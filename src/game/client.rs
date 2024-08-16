use std::collections::HashMap;

use macroquad::prelude::*;
use renet::ClientId;

use crate::{net::client::Client, ClientChannel, ClientInput, EntityId, Player, ServerMessages};

use super::{camera::Viewport, map::{Map, TILE_SIZE}};

struct PlayerResources {
    id: EntityId,
    tex: Texture2D
}

pub struct Game {
    client: Client,
    lobby: HashMap<EntityId, Player>,
    viewport: Viewport,
    map: Map,
    player_res: PlayerResources
}

impl Game {
    pub async fn new() -> Self {
        let (client_id, client) = Client::new("127.0.0.1:6667".parse().unwrap());

        Self {
            client,
            lobby: HashMap::new(),
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
                    println!("Player {} joined", ClientId::from(id));
    
                    self.lobby.insert(id.into(), Player { pos, target_pos: pos });
                }
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} left", ClientId::from(id));
    
                    self.lobby.remove(&id.into());
                }
                ServerMessages::PlayerUpdate { id, pos } => {
                    if let Some(player) = self.lobby.get_mut(&id) {
                        if let Some(tile) = self.map.get_tile(pos.into()) {
                            player.target_pos = tile.rect.center().into();
                        }
                    }
                }
            }
        }
    
        for (player_id, player) in self.lobby.iter_mut() {
            let pos = Vec2::from(player.pos);
            let target_pos = Vec2::from(player.target_pos);
            let speed = 5.0;

            player.pos = pos.lerp(target_pos, speed * get_frame_time()).into();
            
            if self.player_res.id == *player_id {
                let mouse_pos = if is_mouse_button_released(MouseButton::Left) {
                    Some(self.viewport.camera.screen_to_world(mouse_position().into()).into())
                } else {
                    None
                };
                
                let input = &ClientInput {
                    left: is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
                    up: is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
                    down: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
                    right: is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
                    mouse_pos
                };
            
                if input.left || input.up || input.down || input.right || input.mouse_pos.is_some() {
                    let msg = bincode::serialize(input).unwrap();
                    
                    self.client.send(ClientChannel::ClientInput, msg);
                }

                self.map.update(&["base"], Rect::new(
                    player.pos.x - screen_width() / 2.0 - TILE_SIZE.x,
                    player.pos.y - screen_height() / 2.0 - TILE_SIZE.y,
                    screen_width() + TILE_SIZE.x,
                    screen_height() + TILE_SIZE.y
                ));
    
                self.viewport.update(player.pos.x, player.pos.y, screen_width(), screen_height());
            }
        }
    
        self.client.update();
    }    

    pub fn draw(&mut self) {
        self.map.draw();

        for player in self.lobby.values() {
            draw_texture_ex(
                &self.player_res.tex,
                player.pos.x, player.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(32.0, 128.0, TILE_SIZE.x, TILE_SIZE.y)),
                    ..Default::default()
                }
            );
        }

        self.viewport.draw();
    }
}