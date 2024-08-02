mod server;

use std::{thread, time::Duration};

use dyhra::{ClientInput, ClientMessages, Position, ServerChannel, ServerMessages};
use server::Server;

fn main() {
    let mut server = Server::new("127.0.0.1:6667".parse().unwrap());

    loop {
        server.update();
        
        if let Some(client_id) = server.on_client_connect() {
            println!("Client {} connected.", client_id);

            //self.lobby.insert(client_id, player);

            let msg = bincode::serialize(
                &ServerMessages::PlayerCreate {
                    id: client_id.into(),
                    pos: Position {
                        x: 0.0,
                        y: 0.0
                    }
                }
            ).unwrap();
            
            server.message_queue.push_back(msg);
            //self.renet.broadcast_message(ServerChannel::ServerMessages, msg);   
        } else if let Some((client_id, reason)) = server.on_client_disconnect() {
            println!("Client {} disconnected: {}", client_id, reason);
                    
            //self.lobby.remove(&client_id);

            let msg = bincode::serialize(&ServerMessages::PlayerDelete { id: client_id.into() }).unwrap();

            server.message_queue.push_back(msg);
            //self.renet.broadcast_message(ServerChannel::ServerMessages, msg);
        }

        while let Some((client_id, client_input)) = server.get_client_input() {
            match client_input {
                ClientInput { left, up, down, right } => {
                    let x = (right as i8 - left as i8) as f32;
                    let y = (down as i8 - up as i8) as f32;

                    let msg = bincode::serialize(
                        &ServerMessages::PlayerUpdate {
                            id: client_id.into(),
                            pos: Position { x, y }
                        }
                    ).unwrap();
                    
                    server.renet.send_message(client_id, ServerChannel::ServerMessages, msg);
                }
            }
        }

        while let Some((client_id, client_msg)) = server.get_client_msg() {
            match client_msg {
                ClientMessages::PlayerCommand { id } => {
                    // handle commands
                }
            }
        }

        thread::sleep(Duration::from_millis(50));
    }
}