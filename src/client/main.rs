mod client;

use client::Client;
use dyhra::{ClientChannel, ClientInput, ServerMessages};
use macroquad::{color::{RED, SKYBLUE}, input::{is_key_down, KeyCode}, shapes::draw_rectangle, window::{clear_background, next_frame}};
use renet::ClientId;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut client = Client::new("127.0.0.1:6667".parse().unwrap());

    loop {
        clear_background(SKYBLUE);

        client.update();

        if client.renet.is_connected() {
            while let Some(server_msg) = client.get_server_msg() {
                match server_msg {
                    ServerMessages::PlayerCreate { id, pos } => {
                        println!("Player {} joined", ClientId::from(id));


                        draw_rectangle(pos.x, pos.y, 32.0, 32.0, RED);
                    }
                    ServerMessages::PlayerDelete { id } => {}
                    ServerMessages::PlayerUpdate { id, pos } => {
                        draw_rectangle(pos.x, pos.y, 32.0, 32.0, RED);
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

        next_frame().await;
    }
}