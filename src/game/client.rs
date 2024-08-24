use macroquad::{prelude::*, ui::root_ui};

use crate::{net::client::Client, ClientChannel, ClientInput, ClientMessages, EntityId, ServerMessages, Vec2D};

use super::{camera::Viewport, map::{Map, TILE_OFFSET, TILE_SIZE}, world::World};

struct Resources {
    player_id: EntityId,
    player_tex: Texture2D,
    enemy_tex: Texture2D
}

pub struct Game {
    client: Client,
    world: World,
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
            viewport: Viewport::new(screen_width(), screen_height()),
            map: Map::new("assets/map.json", "assets/tiles.png").await,
            res: Resources {
                player_id: client_id.into(),
                player_tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                enemy_tex: load_texture("assets/32rogues/monsters.png").await.unwrap()
            }
        }
    }

    pub fn update(&mut self) {
        while let Some(server_msg) = self.client.get_server_msg() {
            match server_msg {
                ServerMessages::PlayerCreate { id, sprite, pos, health } => {
                    println!("Player {} spawned", id.raw());

                    let player = self.world.spawn_player(id);
                    player.sprite = sprite;
                    player.pos = pos;
                    player.health = health;
                    player.sprite.frame = (1.0, 4.0);
                }
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} despawned", id.raw());
                    
                    self.world.despawn_entity(id);
                }
                ServerMessages::PlayerUpdate { id, pos, target } => {
                    if let Some(player) = self.world.players.get_mut(&id) {
                        if let Some(tile) = self.map.get_tile(pos.into()) {
                            if tile.walkable {
                                player.target_pos = tile.rect.center().into();
                            }
                        }

                        player.target = target;
                    }
                }
                ServerMessages::EnemyCreate { id, sprite, pos, health } => {
                    println!("Enemy {} spawned", id.raw());

                    let enemy = self.world.spawn_enemy();
                    enemy.sprite = sprite;
                    enemy.pos = pos;
                    enemy.health = health;
                },
                ServerMessages::EnemyDelete { id } => {
                    println!("Enemy {} passed away", id.raw());

                    self.world.despawn_entity(id);
                },
                ServerMessages::EnemyUpdate { id, health } => {
                    let enemy = self.world.enemies.get_mut(&id).unwrap();
                    enemy.health = health;
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

        if let Some(player) = self.world.players.get(&self.res.player_id) {
            let tile_pos = player.pos / TILE_SIZE.into();
            
            root_ui().label(None, &format!("Map position: ({:.2}, {:.2})", player.pos.x, player.pos.y));
            root_ui().label(None, &format!("Tile pos: ({:.2}, {:.2})", tile_pos.x, tile_pos.y));
            root_ui().label(None, &format!("Player target pos: ({:.2}, {:.2})", player.target_pos.x, player.target_pos.y));
        }
    }

    fn handle_player_input(&mut self) {
        let mouse_target_pos = if is_mouse_button_released(MouseButton::Left) {
            Some(self.viewport.camera.screen_to_world(mouse_position().into()).into())
        } else {
            None
        };

        let mouse_target = if is_mouse_button_released(MouseButton::Right) {
            let mouse_pos = self.viewport.camera.screen_to_world(mouse_position().into());
        
            self.world.enemies.iter()
                .find_map(|(enemy_id, enemy)| {
                    let enemy_rect = Rect::new(enemy.pos.x, enemy.pos.y, TILE_SIZE.x, TILE_SIZE.y);
                    
                    if enemy_rect.contains(mouse_pos) {
                        Some(*enemy_id)
                    } else {
                        None
                    }
                })
        } else {
            None
        };

        let input = &ClientInput {
            left: is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
            up: is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
            down: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
            right: is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
            mouse_target_pos,
            mouse_target
        };

        if input.left || input.up || input.down || input.right || input.mouse_target_pos.is_some() || input.mouse_target.is_some() {
            self.client.send(ClientChannel::ClientInput, input);
        }
    }

    fn update_entities(&mut self) {
        for (player_id, player) in &mut self.world.players {
            let start_pos = Vec2::from(player.pos);
            let target_pos = Vec2::from(player.target_pos);
            let speed = 2.5;

            player.pos = start_pos.lerp(target_pos, speed * get_frame_time()).into();
            
            if let Some(target) = player.target {
                let msg = ClientMessages::PlayerAttack { target };
                self.client.send(ClientChannel::ClientMessages, msg);
            }

            if self.res.player_id == *player_id {
                self.map.update(&["base", "floor", "props"], Rect::new(
                    player.pos.x - screen_width() / 2.0 - TILE_SIZE.x * 2.0,
                    player.pos.y - screen_height() / 2.0 - TILE_SIZE.y * 2.0,
                    screen_width() + TILE_SIZE.x * 2.0,
                    screen_height() + TILE_SIZE.y * 2.0
                ));
    
                self.viewport.update(player.pos.into(), screen_width(), screen_height());
            }
        }

        for (_, enemy) in self.world.enemies.iter_mut() {
            enemy.pos += Vec2D {
                x: rand::gen_range(-1.0, 1.0),
                y: rand::gen_range(-1.0, 1.0),
            };
        }
    }

    fn draw_entities(&mut self) {
        for (_, entity) in self.world.players.iter().chain(self.world.enemies.iter()) {
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

        for (_, enemy) in self.world.enemies.iter() {
            draw_texture_ex(
                &self.res.enemy_tex,
                enemy.pos.x, enemy.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(
                        enemy.sprite.frame.0 as f32 * TILE_SIZE.x,
                        enemy.sprite.frame.1 as f32 * TILE_SIZE.y,
                        TILE_SIZE.x, TILE_SIZE.y
                    )),
                    ..Default::default()
                }
            );

            enemy.pos.draw_rect(TILE_SIZE, RED);
        }

        for (player_id, player) in self.world.players.iter() {
            draw_texture_ex(
                &self.res.player_tex,
                player.pos.x, player.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(
                        player.sprite.frame.0 as f32 * TILE_SIZE.x,
                        player.sprite.frame.1 as f32 * TILE_SIZE.y,
                        TILE_SIZE.x, TILE_SIZE.y
                    )),
                    ..Default::default()
                }
            );

            if *player_id == self.res.player_id {
                player.target_pos.draw_rect(TILE_SIZE, BLUE);
                player.pos.draw_rect(TILE_SIZE, GREEN);

                if let Some(player_target) = player.target {
                    if let Some(target) = self.world.enemies.get(&player_target) {
                        target.pos.draw_rect(TILE_SIZE, ORANGE);
                    }
                }
            }
        }
    }
}