use std::collections::{HashMap, VecDeque};

use crate::{net::server::Server, ClientMessages, EntityId, Player, Position, ServerMessages};

pub struct Game {
    server: Server,
    msg_queue: VecDeque<ServerMessages>,
    lobby: HashMap<EntityId, Player>
}

impl Game {
    pub fn new() -> Self {
        Self {
            server: Server::new("127.0.0.1:6667".parse().unwrap()),
            msg_queue: VecDeque::new(),
            lobby: HashMap::new()
        }
    }

    pub fn update(&mut self) {
        if let Some(client_id) = self.server.on_client_connect() {
            println!("Client {} connected.", client_id);

            for (id, other_player) in &self.lobby {
                let msg = ServerMessages::PlayerCreate {
                    id: *id,
                    pos: other_player.pos,
                };
                self.server.send(client_id, msg);
            }

            let player = Player::default();
            self.lobby.insert(client_id.into(), player);

            let msg = ServerMessages::PlayerCreate {
                id: client_id.into(),
                pos: player.pos
            };
            self.msg_queue.push_back(msg);
        } else if let Some((client_id, reason)) = self.server.on_client_disconnect() {
            println!("Client {} disconnected: {}", client_id, reason);
                    
            self.lobby.remove(&client_id.into());

            let msg = ServerMessages::PlayerDelete { id: client_id.into() };
            self.msg_queue.push_back(msg);
        }

        while let Some((client_id, input)) = self.server.get_client_input() {
            let player = self.lobby.get_mut(&client_id.into()).unwrap();
            let x = (input.right as i8 - input.left as i8) as f32;
            let y = (input.down as i8 - input.up as i8) as f32;

            if let Some(mouse_pos) = input.mouse_pos {
                player.pos = mouse_pos;
            } else {
                player.pos += Position { x, y };
            }

            let msg = &ServerMessages::PlayerUpdate {
                id: client_id.into(),
                pos: player.pos
            };
            self.server.broadcast(msg);
        }

        while let Some((client_id, client_msg)) = self.server.get_client_msg() {
            match client_msg {
                ClientMessages::PlayerCommand { id } => {
                    // handle commands
                }
            }
        }

        if let Some(msg) = self.msg_queue.pop_front() {
            self.server.broadcast(msg);
        }

        self.server.update(20);
    }
}