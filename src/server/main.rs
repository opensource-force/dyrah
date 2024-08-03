mod server;

use std::{thread, time::Duration};

use dyhra::{ClientMessages, Player, Position, SerializableClientId, ServerChannel, ServerMessages};
use server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1:6667".parse().unwrap());

    loop {        
        if let Some(client_id) = server.on_client_connect() {
            println!("Client {} connected.", client_id);

            let player = Player {
                pos: Position {
                    x: 0.0,
                    y: 0.0,
                }
            };

            server.lobby.insert(client_id, player);

            let msg = bincode::serialize(
                &ServerMessages::PlayerCreate {
                    id: client_id.into(),
                    pos: player.pos
                }
            ).unwrap();
            
            server.message_queue.push_back(msg);
            //self.renet.broadcast_message(ServerChannel::ServerMessages, msg);

            for (id, other_player) in &server.lobby {
                let update_msg = bincode::serialize(
                    &ServerMessages::PlayerCreate {
                        id: SerializableClientId::from(*id),
                        pos: other_player.pos,
                    }
                ).unwrap();

                server.renet.send_message(client_id, ServerChannel::ServerMessages, update_msg);
            }

        } else if let Some((client_id, reason)) = server.on_client_disconnect() {
            println!("Client {} disconnected: {}", client_id, reason);
                    
            server.lobby.remove(&client_id);

            let msg = bincode::serialize(&ServerMessages::PlayerDelete { id: client_id.into() }).unwrap();

            server.message_queue.push_back(msg);
            //self.renet.broadcast_message(ServerChannel::ServerMessages, msg);
        }

        while let Some((client_id, input)) = server.get_client_input() {
            let player = server.lobby.get_mut(&client_id).unwrap();

            let x = (input.right as i8 - input.left as i8) as f32;
            let y = (input.down as i8 - input.up as i8) as f32;
             
            player.pos.x += x;
            player.pos.y += y;

            let msg = bincode::serialize(
                &ServerMessages::PlayerUpdate {
                    id: client_id.into(),
                    pos: Position {
                        x: player.pos.x,
                        y: player.pos.y
                    }
                }
            ).unwrap();
            
            server.renet.broadcast_message(ServerChannel::ServerMessages, msg);
        }

        while let Some((client_id, client_msg)) = server.get_client_msg() {
            match client_msg {
                ClientMessages::PlayerCommand { id } => {
                    // handle commands
                }
            }
        }

        server.update();

        thread::sleep(Duration::from_millis(50));
    }
}