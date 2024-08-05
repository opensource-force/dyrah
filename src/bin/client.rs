use dyhra::{game::map::{Map, TILE_SIZE}, net::client::Client, ClientChannel, ClientInput, ServerMessages};
use macroquad::prelude::*;
use renet::ClientId;

#[macroquad::main("Dyhra")]
async fn main() {
    let (client_id, mut client) = Client::new("127.0.0.1:6667".parse().unwrap());
    let mut map = Map::new("assets/map.json", "assets/tiles.png").await;
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.0, 0.0, screen_width(), -screen_height()
    ));
    let player_texture = load_texture("assets/32rogues/rogues.png").await.unwrap();

    loop {
        clear_background(SKYBLUE);

        if client.renet.is_connected() {
            while let Some(server_msg) = client.get_server_msg() {
                match server_msg {
                    ServerMessages::PlayerCreate { id, player } => {
                        println!("Player {} joined", ClientId::from(id));

                        client.lobby.insert(id.into(), player);
                    }
                    ServerMessages::PlayerDelete { id } => {
                        println!("Player {} left", ClientId::from(id));

                        client.lobby.remove(&id.into());
                    }
                    ServerMessages::PlayerUpdate { id, pos } => {
                        if let Some(player) = client.lobby.get_mut(&id.into()) {
                            player.pos = pos;

                            if ClientId::from(id) == client_id {
                                map.update(&["base"], Rect::new(
                                    player.pos.x - screen_width() / 2.0 - TILE_SIZE.x,
                                    player.pos.y - screen_height() / 2.0 - TILE_SIZE.y,
                                    screen_width() + TILE_SIZE.x,
                                    screen_height() + TILE_SIZE.y,
                                ));
    
                                camera.target = vec2(player.pos.x, player.pos.y);
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
                client.renet.send_message(ClientChannel::ClientInput, msg);
            }
        }

        map.draw();

        for player in client.lobby.values() {
            draw_texture_ex(&player_texture, player.pos.x, player.pos.y, WHITE, DrawTextureParams {
                source: Some(Rect::new(0.0 * TILE_SIZE.x, 4.0 * TILE_SIZE.y, TILE_SIZE.x, TILE_SIZE.y)),
                ..Default::default()
            })
        }

        set_camera(&camera);

        client.update();

        next_frame().await;
    }
}