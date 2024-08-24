use std::collections::VecDeque;

use crate::{net::server::Server, ClientMessages, ServerMessages, Vec2D};

use super::{map::TILE_SIZE, world::World};

pub struct Game {
    server: Server,
    msg_queue: VecDeque<ServerMessages>,
    world: World
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::default();

        for i in 1..4 {
            let enemy = world.spawn_enemy();
            enemy.sprite.frame = (3.0, 4.0);
            enemy.pos = Vec2D { x: i as f32 * TILE_SIZE.x, y: i as f32 * TILE_SIZE.y };
            enemy.health = 80.0;
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

            for (id, enemy) in &self.world.enemies {
                let msg = ServerMessages::EnemyCreate {
                    id: *id,
                    sprite: enemy.sprite,
                    pos: enemy.pos,
                    health: enemy.health
                };
                self.server.send(client_id, msg);
            }

            for (id, other_player) in &self.world.players {
                let msg = ServerMessages::PlayerCreate {
                    id: *id,
                    sprite: other_player.sprite,
                    pos: other_player.pos,
                    health: other_player.health
                };
                self.server.send(client_id, msg);
            }

            let player = self.world.spawn_player(client_id.into());
            let msg = ServerMessages::PlayerCreate {
                id: client_id.into(),
                sprite: player.sprite,
                pos: player.pos,
                health: 100.0
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
                    if let Some(enemy) = self.world.enemies.get_mut(&target) {
                        enemy.health -= 10.0;
                        println!("Enemy health: {}", enemy.health);

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
            let player = self.world.players.get_mut(&client_id.into()).unwrap();
    
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
            self.server.broadcast(msg);
        }
    }
    
}