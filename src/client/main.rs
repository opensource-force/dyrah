mod client;

use client::Client;
use dyhra::{ClientChannel, ClientInput, Player, ServerMessages};
use macroquad::{color::{RED, SKYBLUE}, input::{is_key_down, KeyCode}, shapes::draw_rectangle, window::{clear_background, next_frame}};
use renet::ClientId;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut client = Client::new("127.0.0.1:6667".parse().unwrap());


    loop {
        clear_background(SKYBLUE);

        if client.renet.is_connected() {
            while let Some(server_msg) = client.get_server_msg() {
                match server_msg {
                    ServerMessages::PlayerCreate { id, pos } => {
                        println!("Player {} joined", ClientId::from(id));

                        client.lobby.insert(id.into(), Player { pos });
                    }
                    ServerMessages::PlayerDelete { id } => {
                        println!("Player {} left", ClientId::from(id));

                        client.lobby.remove(&id.into());
                    }
                    ServerMessages::PlayerUpdate { id, pos } => {
                        // sync player pos with server player pos
                        if let Some(player) = client.lobby.get_mut(&id.into()) {
                            player.pos = pos;
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
                client.renet.send_message(ClientChannel::ClientInput, msg);
            }
        }

        for player in client.lobby.values() {
            draw_rectangle(player.pos.x, player.pos.y, 32.0, 32.0, RED);
        }

        client.update();

        next_frame().await;
    }
}