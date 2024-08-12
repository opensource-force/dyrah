use std::collections::HashMap;

use macroquad::prelude::*;
use renet::ClientId;

use crate::{net::client::Client, ClientChannel, ClientInput, EntityId, Position, ServerMessages};

use super::{camera::Viewport, map::{Map, TILE_SIZE}};

struct PlayerResources {
    id: EntityId,
    tex: Texture2D
}

struct ClientPlayer {
    pos: Position,
    target_pos: Position
}

pub struct Game {
    client: Client,
    lobby: HashMap<EntityId, ClientPlayer>,
    camera: Viewport,
    map: Map,
    player_res: PlayerResources
}

impl Game {
    pub async fn new() -> Self {
        let (client_id, client) = Client::new("127.0.0.1:6667".parse().unwrap());

        Self {
            client,
            lobby: HashMap::new(),
            camera: Viewport::new(screen_width(), screen_height()),
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

                    self.lobby.insert(id.into(), ClientPlayer { pos, target_pos: pos });
                }
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} left", ClientId::from(id));

                    self.lobby.remove(&id.into());
                }
                ServerMessages::PlayerUpdate { id, pos } => {
                    if let Some(player) = self.lobby.get_mut(&id) {
                        if let Some(tile) = self.map.get_tile(pos.x, pos.y) {
                            let tile_center = tile.rect.center();

                            player.target_pos.x = tile_center.x;
                            player.target_pos.y = tile_center.y;
                        }
                    }
                }
            }
        }

        let input = &ClientInput {
            left: is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
            up: is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
            down: is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),
            right: is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)
        };

        if input.left || input.up || input.down || input.right {
            let msg = bincode::serialize(input).unwrap();
            
            self.client.send(ClientChannel::ClientInput, msg);
        }

        for (player_id, player) in self.lobby.iter_mut() {
            let pos = Vec2::from(player.pos);
            let target_pos = Vec2::from(player.target_pos);

            if pos != target_pos {
                let direction = (target_pos - pos).normalize();
                let speed = 100.0;
                let movement = direction * speed * get_frame_time();

                if (pos + movement).distance(target_pos) < speed * get_frame_time() {
                    player.pos = player.target_pos;
                } else {
                    player.pos.x += movement.x;
                    player.pos.y += movement.y;
                }
            }

            if self.player_res.id == *player_id {
                self.camera.update(player.pos.x, player.pos.y, screen_width(), screen_height());
                self.map.update(&["base"], Rect::new(
                    player.pos.x - screen_width() / 2.0 - TILE_SIZE.x,
                    player.pos.y - screen_height() / 2.0 - TILE_SIZE.y,
                    screen_width() + TILE_SIZE.x,
                    screen_height() + TILE_SIZE.y
                ));
            }
        }

        self.client.update();
    }

    pub fn draw(&mut self) {
        self.camera.draw();
        self.map.draw();

        for player in self.lobby.values() {
            draw_texture_ex(
                &self.player_res.tex,
                player.pos.x, player.pos.y,
                WHITE, DrawTextureParams {
                    source: Some(Rect::new(64.0, 96.0, TILE_SIZE.x, TILE_SIZE.y)),
                    ..Default::default()
                }
            );
        }
    }
}