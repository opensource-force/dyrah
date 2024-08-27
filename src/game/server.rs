use std::collections::VecDeque;

use macroquad::rand::gen_range;

use crate::{game::world::{Entity, EntityKind}, net::server::Server, ClientMessages, ServerMessages, Vec2D};

use super::{map::{TILE_OFFSET, TILE_SIZE}, world::World};

pub struct Game {
    server: Server,
    msg_queue: VecDeque<ServerMessages>,
    world: World
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::default();

        for _ in 1..99 {
            let enemy = Entity {
                kind: EntityKind::Enemy,
                pos: Vec2D {
                    x: gen_range(TILE_OFFSET.x, 64.0 * TILE_SIZE.x),
                    y: gen_range(TILE_OFFSET.y, 64.0 * TILE_SIZE.y)
                },
                health: gen_range(50, 80) as f32,
                ..Default::default()
            };
            world.spawn_entity(enemy);
        }

        Self {
            server: Server::new("127.0.0.1:6667".parse().unwrap()),
            msg_queue: VecDeque::new(),
            world
        }
    }

    pub fn update(&mut self) {
        if let Some(client_id) = self.server.on_client_connect() {
            println!("Client {} connected.", client_id);

            for (enemy_id, enemy) in self.world.enemies() {
                let msg = ServerMessages::EnemyCreate {
                    id: *enemy_id,
                    pos: enemy.pos,
                    health: enemy.health
                };
                self.server.send(client_id, msg);
            }

            for (opp_player_id, opp_player) in self.world.players() {
                let msg = ServerMessages::PlayerCreate {
                    id: *opp_player_id,
                    pos: opp_player.pos,
                    health: opp_player.health
                };
                self.server.send(client_id, msg);
            }

            let player = Entity {
                kind: EntityKind::Player,
                pos: TILE_SIZE.into(),
                health: 100.0,
                ..Default::default()
            };
            self.world.spawn_entity_from_id(client_id.into(), player);

            let msg = ServerMessages::PlayerCreate {
                id: client_id.into(),
                pos: player.pos,
                health: player.health
            };
            self.msg_queue.push_back(msg);
        } else if let Some((client_id, reason)) = self.server.on_client_disconnect() {
            println!("Client {} disconnected: {}", client_id, reason);
                    
            self.world.despawn_entity(client_id.into());

            let msg = ServerMessages::PlayerDelete { id: client_id.into() };
            self.msg_queue.push_back(msg);
        }

        self.handle_player_input();

        while let Some((_client_id, client_msg)) = self.server.get_client_msg() {
            match client_msg {
                ClientMessages::PlayerAttack { target } => {
                    if let Some(enemy) = self.world.get_entity_mut(target) {
                        enemy.health -= 10.0;
        
                        let msg = ServerMessages::EnemyUpdate { id: target, health: enemy.health };
                        self.msg_queue.push_back(msg);

                        if enemy.health <= 0.0 {
                            self.world.despawn_entity(target);
    
                            let msg = ServerMessages::EnemyDelete { id: target };
                            self.msg_queue.push_back(msg);
                        }
                    }
                }
            }
        }

        if let Some(msg) = self.msg_queue.pop_front() {
            self.server.broadcast(msg);
        }

        self.server.update(20);
    }

    fn handle_player_input(&mut self) {
        while let Some((client_id, input)) = self.server.get_client_input() {
            if let Some(player) = self.world.get_entity_mut(client_id.into()) {
                player.vel.x = (input.right as i8 - input.left as i8) as f32;
                player.vel.y = (input.down as i8 - input.up as i8) as f32;
    
                if let Some(mouse_target_pos) = input.mouse_target_pos {
                    player.pos = mouse_target_pos;
                } else {
                    player.pos += player.vel * TILE_SIZE.into();
                }
    
                if let Some(mouse_target) = input.mouse_target {
                    player.target = Some(mouse_target);
                }
        
                let msg = ServerMessages::PlayerUpdate {
                    id: client_id.into(),
                    pos: player.pos,
                    target: player.target
                };
                self.msg_queue.push_back(msg);
            }
        }
    }
    
}