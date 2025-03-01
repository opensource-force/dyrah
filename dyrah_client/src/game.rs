use bincode::{deserialize, serialize};
use macroquad::{prelude::*, ui::root_ui};
use secs::prelude::{ExecutionMode, World};
use wrym::{client::{Client, ClientEvent}, transport::LaminarTransport};

use dyrah_shared::{ClientMessage, Position, ServerMessage};

use crate::{camera::Camera, map::TiledMap, PlayerSprite};

const TILE_SIZE: Vec2 = vec2(32., 32.);

pub struct Game {
    client: Client<LaminarTransport>,
    world: World,
    map: TiledMap,
    map_texture: Texture2D,
    camera: Camera,
    player_id: Option<u64>
}

// systems
fn render_system(world: &World) {
    for (_, (pos,)) in world.query::<(&Position,)>() {
        let player_spr = world.get_resource::<PlayerSprite>().unwrap();

        draw_texture_ex(
            &player_spr.texture, pos.x, pos.y, WHITE, DrawTextureParams {
                source: Some(Rect::new(
                    player_spr.frame.0, player_spr.frame.1, TILE_SIZE.x, TILE_SIZE.y
                )),
                ..Default::default()
            }
        );
    }
}

impl Game {
    pub async fn new() -> Self {
        let transport = LaminarTransport::new("127.0.0.1:0");
        let mut world = World::default();
        let map = TiledMap::new("assets/map.json");
        let player_tex = load_texture("assets/32rogues/rogues.png").await.unwrap();

        set_default_filter_mode(FilterMode::Nearest);

        world.add_resource(PlayerSprite { texture: player_tex, frame: (0., 0.) });
        
        world.add_system(render_system, ExecutionMode::Parallel);

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            world,
            map,
            map_texture: load_texture("assets/tiles.png").await.unwrap(),
            camera: Camera::default(),
            player_id: None
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.client.recv_event() {
            match event {
                ClientEvent::Connected => {
                    println!("Connected to server!!");
                }
                ClientEvent::Disconnected => {
                    println!("Lost connection to server");
                }
                ClientEvent::MessageReceived(bytes) => {
                    let server_msg = deserialize::<ServerMessage>(&bytes).unwrap();

                    match server_msg {
                        ServerMessage::PlayerConnected { id, pos } => {
                            self.world.spawn((pos,));

                            if self.player_id.is_none() {
                                self.player_id = Some(id);
                            }
                        }
                        ServerMessage::PlayerMoved { id, pos } => {
                            for (entity, (position,)) in self.world.query::<(&mut Position,)>() {
                                if entity.to_bits() == id {
                                    let start_pos = vec2(position.x, position.y);
                                    let target_pos = vec2(pos.x, pos.y);
                                    let speed = 10.;

                                    position.x = start_pos.x.lerp(target_pos.x, speed * get_frame_time());
                                    position.y = start_pos.y.lerp(target_pos.y, speed * get_frame_time());

                                    self.camera.attach_sized(
                                        position.x, position.y, screen_width(), screen_height()
                                    );
                                    self.camera.set();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        self.client.poll();
        self.handle_events();

        root_ui().label(None, &format!("FPS: {}", get_fps()));

        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);

        if left || up || down || right {
            let msg = ClientMessage::PlayerMove { left, up, right, down };
            self.client.send(&serialize(&msg).unwrap());
        }
    }
    
    pub async fn run(&mut self) {
        loop {
            clear_background(SKYBLUE);

            self.update();
            self.map.draw_tiles(&self.map_texture);
            self.world.run_systems();

            next_frame().await;
        }
    }
}