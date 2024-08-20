use macroquad::prelude::*;

use crate::{net::client::Client, ClientChannel, ClientInput, EntityId, Position, ServerMessages};

use super::{camera::Viewport, map::{Map, TILE_OFFSET, TILE_SIZE}, world::World};

struct PlayerResources {
    id: EntityId,
    tex: Texture2D
}

pub struct Game {
    client: Client,
    world: World,
    viewport: Viewport,
    map: Map,
    player_res: PlayerResources
}

impl Game {
    pub async fn new() -> Self {
        let (client_id, client) = Client::new("127.0.0.1:6667".parse().unwrap());

        Self {
            client,
            world: World::default(),
            viewport: Viewport::new(screen_width(), screen_height()),
            map: Map::new("assets/map.json", "assets/tiles.png").await,
            player_res: PlayerResources {
                id: client_id.into(),
                tex: load_texture("assets/32rogues/rogues.png").await.unwrap()
            }
        }
    }

    pub fn update(&mut self) {
        while let Some(server_msg) = self.client.get_server_msg() {
            match server_msg {
                ServerMessages::PlayerCreate { id, pos } => {
                    println!("Player {} spawned", id.raw());

                    self.world.spawn_player_at(id, pos);
                }
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} despawned", id.raw());
                    
                    self.world.despawn_entity(id);
                }
                ServerMessages::PlayerUpdate { id, pos, target } => {
                    if let Some(player) = self.world.players.get_mut(&id) {
                        if let Some(tile) = self.map.get_tile(pos.into()) {
                            player.target_pos = tile.rect.center().into();
                        }

                        player.target = target;
                    }
                }
                ServerMessages::EnemyCreate { id, pos } => {
                    println!("Enemy {} spawned", id.raw());

                    self.world.spawn_enemy_at(pos);
                }
            }
        }
    
        for (player_id, player) in &mut self.world.players {
            let start_pos = Vec2::from(player.pos);
            let target_pos = Vec2::from(player.target_pos);
            let speed = 5.0;

            player.pos = start_pos.lerp(target_pos, speed * get_frame_time()).into();
            
            if self.player_res.id == *player_id {
                let mouse_pos = if is_mouse_button_released(MouseButton::Left) {
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
                    mouse_pos,
                    mouse_target
                };
            
                if input.left || input.up || input.down || input.right || input.mouse_pos.is_some() {
                    let msg = bincode::serialize(input).unwrap();
                    
                    self.client.send(ClientChannel::ClientInput, msg);
                }

                self.map.update(&["base"], Rect::new(
                    player.pos.x - screen_width() / 2.0 - TILE_SIZE.x * 2.0,
                    player.pos.y - screen_height() / 2.0 - TILE_SIZE.y * 2.0,
                    screen_width() + TILE_SIZE.x * 2.0,
                    screen_height() + TILE_SIZE.y * 2.0
                ));
    
                self.viewport.update(player.pos.into(), screen_width(), screen_height());
            }
        }

        for (_, enemy) in self.world.enemies.iter_mut() {
            enemy.pos += Position {
                x: rand::gen_range(-1.0, 1.0),
                y: rand::gen_range(-1.0, 1.0),
            };
        }
    
        self.client.update();
    }    

    pub fn draw(&mut self) {
        self.map.draw();

        for (_, enemy) in self.world.enemies.iter() {
            draw_texture_ex(
                &self.player_res.tex,
                enemy.pos.x, enemy.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(64.0, 128.0, TILE_SIZE.x, TILE_SIZE.y)),
                    ..Default::default()
                }
            );

            draw_rectangle_lines(enemy.pos.x, enemy.pos.y, TILE_SIZE.x, TILE_SIZE.y, 2.0, RED);
        }

        for (player_id, player) in self.world.players.iter() {
            draw_texture_ex(
                &self.player_res.tex,
                player.pos.x, player.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(32.0, 128.0, TILE_SIZE.x, TILE_SIZE.y)),
                    ..Default::default()
                }
            );

            if *player_id == self.player_res.id {
                draw_rectangle_lines(player.target_pos.x, player.target_pos.y, TILE_SIZE.x, TILE_SIZE.y, 2.0, BLUE);
                draw_rectangle_lines(player.pos.x, player.pos.y, TILE_SIZE.x, TILE_SIZE.y, 2.0, GREEN); 

                if let Some(player_target) = self.world.enemies.get(&player.target) {
                    draw_rectangle_lines(player_target.pos.x, player_target.pos.y, TILE_SIZE.x, TILE_SIZE.y, 2.0, ORANGE);
                }
            }
        }

        self.viewport.draw();

        let mouse_pos = self.viewport.camera.screen_to_world(mouse_position().into());
        draw_rectangle_lines(
            mouse_pos.x - TILE_OFFSET.x,
            mouse_pos.y - TILE_OFFSET.y,
            TILE_SIZE.x, TILE_SIZE.y,
            2.0, PURPLE
        );
    }
}