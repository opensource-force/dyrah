use macroquad::{prelude::*, ui::root_ui};

use crate::{game::{world::Entity, Sprite}, net::client::Client, ClientChannel, ClientInput, ClientMessages, EntityId, ServerMessages, Vec2D};

use super::{camera::Viewport, map::{Map, TILE_OFFSET, TILE_SIZE}, world::World, Lobby};

struct Resources {
    player_id: EntityId,
    player_view: Rect,
    player_tex: Texture2D,
    enemy_tex: Texture2D
}

pub struct Game {
    client: Client,
    world: World,
    lobby: Lobby,
    viewport: Viewport,
    map: Map,
    res: Resources
}

impl Game {
    pub async fn new() -> Self {
        let (client_id, client) = Client::new("127.0.0.1:6667".parse().unwrap());

        Self {
            client,
            world: World::default(),
            lobby: Lobby::default(),
            viewport: Viewport::new(screen_width(), screen_height()),
            map: Map::new("assets/map.json", "assets/tiles.png").await,
            res: Resources {
                player_id: client_id.into(),
                player_view: Rect::default(),
                player_tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                enemy_tex: load_texture("assets/32rogues/monsters.png").await.unwrap()
            }
        }
    }

    pub fn update(&mut self) {
        while let Some(server_msg) = self.client.get_server_msg() {
            match server_msg {
                ServerMessages::PlayerCreate { id, pos, health } => {
                    println!("Player {} spawned", id.raw());

                    let player = Entity {
                        sprite: Sprite { frame: Vec2D { x: 1.0, y: 4.0 } },
                        pos,
                        health,
                        ..Default::default()
                    };
                    let player_idx = self.world.spawn_entity(player);
                    self.lobby.players.insert(id, player_idx);

                    self.res.player_view = player.pos.screen_rect();
                    self.map.update(&["base", "floor", "props"], self.res.player_view);
                    self.viewport.update(player.pos.into(), screen_width(), screen_height());
                }
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} despawned", id.raw());
                    
                    if let Some(player_idx) = self.lobby.players.remove(&id) {
                        self.world.despawn_entity(player_idx);
                    }
                }
                ServerMessages::PlayerUpdate { id, pos, target } => {
                    if let Some(player_idx) = self.lobby.players.get(&id) {
                        let player = &mut self.world.entities[*player_idx];

                        if let Some(tile) = self.map.get_tile(pos.into()) {
                            if tile.walkable {
                                player.target_pos = Some(tile.rect.center().into());
                            }
                        }

                        player.target = target;
                    }
                }
                ServerMessages::EnemyCreate { id, pos, health } => {
                    println!("Enemy {} spawned", id.raw());

                    let enemy = Entity {
                        sprite: Sprite { frame: Vec2D {
                            x: rand::gen_range(0, 1) as f32,
                            y: rand::gen_range(0, 7) as f32
                        }},
                        pos,
                        health,
                        ..Default::default()
                    };
                    let enemy_idx = self.world.spawn_entity(enemy);
                    self.lobby.enemies.insert(id, enemy_idx);
                },
                ServerMessages::EnemyDelete { id } => {
                    println!("Enemy {} passed away", id.raw());

                    if let Some(idx) = self.lobby.enemies.remove(&id) {
                        self.world.despawn_entity(idx);
                    }
                },
                ServerMessages::EnemyUpdate { id, health } => {
                    if let Some(enemy_idx) = self.lobby.enemies.get(&id) {
                        let enemy = &mut self.world.entities[*enemy_idx];

                        enemy.health = health;
                    }
                }
            }
        }

        self.handle_player_input();
        self.update_entities();
        self.client.update();
    }    

    pub fn draw(&mut self) {
        self.map.draw();
        self.draw_entities();
        self.viewport.draw();

        let mouse_pos = self.viewport.camera.screen_to_world(mouse_position().into());
        draw_rectangle_lines(
            mouse_pos.x - TILE_OFFSET.x,
            mouse_pos.y - TILE_OFFSET.y,
            TILE_SIZE.x, TILE_SIZE.y,
            2.0, PURPLE
        );

        root_ui().label(None, &format!("FPS: {:.1}", get_fps()));
        root_ui().label(None, &format!("Mouse pos: ({:.2}, {:.2})", mouse_pos.x, mouse_pos.y));

        if let Some(player_idx) = self.lobby.players.get(&self.res.player_id) {
            let player = self.world.entities[*player_idx];

            let tile_pos = player.pos / TILE_SIZE.into();
            
            root_ui().label(None, &format!("Map position: ({:.2}, {:.2})", player.pos.x, player.pos.y));
            root_ui().label(None, &format!("Tile pos: ({:.2}, {:.2})", tile_pos.x, tile_pos.y));

            if let Some(target_pos) = player.target_pos {
                root_ui().label(None, &format!("Player target pos: ({:.2}, {:.2})", target_pos.x, target_pos.y));
            }
        }
    }

    fn handle_player_input(&mut self) {
        let mut mouse_target = None;
        let mut mouse_target_pos = None;
        let mouse_world_pos = self.viewport.camera.screen_to_world(mouse_position().into());

        if is_mouse_button_released(MouseButton::Left) {
            mouse_target_pos = Some(mouse_world_pos.into());
        }
    
        if is_mouse_button_released(MouseButton::Right) {
            for (enemy_id, enemy_idx) in &self.lobby.enemies {
                if let Some(enemy) = self.world.entities.get(*enemy_idx) {
                    let enemy_rect = enemy.pos.new_rect(TILE_SIZE);
                    
                    if enemy_rect.contains(mouse_world_pos) {
                        mouse_target = Some(*enemy_id);
                        break;
                    }
                }
            }
        }

        let input = &ClientInput {
            left: is_key_released(KeyCode::A) || is_key_released(KeyCode::Left),
            up: is_key_released(KeyCode::W) || is_key_released(KeyCode::Up),
            down: is_key_released(KeyCode::S) || is_key_released(KeyCode::Down),
            right: is_key_released(KeyCode::D) || is_key_released(KeyCode::Right),
            mouse_target_pos,
            mouse_target
        };

        if input.left || input.up || input.down || input.right || input.mouse_target_pos.is_some() || input.mouse_target.is_some() {
            self.client.send(ClientChannel::ClientInput, input);
        }
    }

    fn update_entities(&mut self) {
        for (player_id, player_idx) in &self.lobby.players {
            let player = &mut self.world.entities[*player_idx];

            if let Some(target_pos) = player.target_pos {
                let start_pos = Vec2::from(player.pos);
                let speed = 2.5;

                player.pos = start_pos.lerp(target_pos.into(), speed * get_frame_time()).into();

                if self.res.player_id == *player_id {
                    self.res.player_view = player.pos.screen_rect();
                    self.map.update(&["base", "floor", "props"], self.res.player_view);
                    self.viewport.update(player.pos.into(), screen_width(), screen_height());
                }
            }
            
            if let Some(target) = player.target {
                let msg = ClientMessages::PlayerAttack { id: *player_id, enemy_id: target };
                self.client.send(ClientChannel::ClientMessages, msg);
            }
        }

        for (_, enemy_idx) in &self.lobby.enemies {
            let enemy = &mut self.world.entities[*enemy_idx];

            enemy.pos += Vec2D {
                x: rand::gen_range(-1.0, 1.0),
                y: rand::gen_range(-1.0, 1.0),
            };
        }
    }

    fn draw_entities(&mut self) {
        for (_, entity) in &self.world.entities {
            if self.res.player_view.contains(entity.pos.into()) {
                draw_rectangle(
                    entity.pos.x,
                    entity.pos.y - 4.0,
                    TILE_SIZE.x,
                    4.0,
                    DARKGRAY,
                );

                draw_rectangle(
                    entity.pos.x,
                    entity.pos.y - 4.0,
                    (TILE_SIZE.x * entity.health / 100.0).clamp(0.0, TILE_SIZE.x),
                    4.0,
                    RED,
                );
            }
        }

        for (_, enemy_idx) in &self.lobby.enemies {
            let enemy = self.world.entities[*enemy_idx];

            if self.res.player_view.contains(enemy.pos.into()) {
                draw_texture_ex(
                    &self.res.enemy_tex,
                    enemy.pos.x, enemy.pos.y,
                    WHITE, DrawTextureParams {
                        source: Some((enemy.sprite.frame * TILE_SIZE.into()).new_rect(TILE_SIZE)),
                        ..Default::default()
                    }
                );
    
                enemy.pos.draw_rect(TILE_SIZE, RED);
            }
        }

        for (player_id, player_idx) in &self.lobby.players {
            let player = self.world.entities[*player_idx];

            if self.res.player_view.contains(player.pos.into()) {
                draw_texture_ex(
                    &self.res.player_tex,
                    player.pos.x, player.pos.y,
                    WHITE, DrawTextureParams {
                        source: Some((player.sprite.frame * TILE_SIZE.into()).new_rect(TILE_SIZE)),
                        ..Default::default()
                    }
                );
            }

            if *player_id == self.res.player_id {
                if let Some(target_pos) = player.target_pos {
                    target_pos.draw_rect(TILE_SIZE, BLUE);
                }
                
                player.pos.draw_rect(TILE_SIZE, GREEN);

                if let Some(player_target) = player.target {
                    if let Some(target_idx) = self.lobby.enemies.get(&player_target) {
                        let target = self.world.entities[*target_idx];
                        
                        target.pos.draw_rect(TILE_SIZE, ORANGE);
                    }
                }
            }
        }
    }
}