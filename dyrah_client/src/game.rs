use bincode::{deserialize, serialize};
use macroquad::{prelude::*, ui::root_ui};
use secs::World;
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{ClientMessage, Position, ServerMessage, TargetPosition, map::TILE_SIZE};

use crate::{PlayerSprite, camera::Camera, map::TiledMap};

fn render_system(world: &World) {
    world.query::<(&Position, &TargetPosition)>(|_, (pos, target_pos)| {
        let player_spr = world.get_resource::<PlayerSprite>().unwrap();

        draw_texture_ex(
            &player_spr.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    player_spr.frame.0,
                    player_spr.frame.1,
                    TILE_SIZE,
                    TILE_SIZE,
                )),
                ..Default::default()
            },
        );

        draw_rectangle_lines(pos.x, pos.y, TILE_SIZE, TILE_SIZE, 2.0, BLUE);
        draw_rectangle_lines(
            target_pos.x,
            target_pos.y,
            TILE_SIZE,
            TILE_SIZE,
            2.0,
            ORANGE,
        );
    });
}

pub struct Game {
    client: Client<Transport>,
    world: World,
    map: TiledMap,
    map_texture: Texture2D,
    camera: Camera,
    player_id: Option<u64>,
    frame_time: f32,
}

impl Game {
    pub async fn new() -> Self {
        let transport = Transport::new("127.0.0.1:0");
        let mut world = World::default();
        let map = TiledMap::new("assets/map.json");
        let player_tex = load_texture("assets/32rogues/rogues.png").await.unwrap();

        set_default_filter_mode(FilterMode::Nearest);

        world.add_resource(PlayerSprite {
            texture: player_tex,
            frame: (0., 0.),
        });

        world.add_system(render_system);

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            world,
            map,
            map_texture: load_texture("assets/tiles.png").await.unwrap(),
            camera: Camera::default(),
            player_id: None,
            frame_time: get_frame_time(),
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
                        ServerMessage::PlayerConnected { id, position } => {
                            self.world
                                .spawn((position, TargetPosition { x: 0., y: 0. }));

                            if self.player_id.is_none() {
                                self.player_id = Some(id);
                            }
                        }
                        ServerMessage::PlayerMoved {
                            id,
                            target_position,
                        } => {
                            self.world.query::<(&mut Position, &mut TargetPosition)>(
                                |entity, (pos, target_pos)| {
                                    if entity.id() == id {
                                        *target_pos = target_position;

                                        self.camera.attach_sized(
                                            pos.x,
                                            pos.y,
                                            screen_width(),
                                            screen_height(),
                                        );
                                        self.camera.set();
                                    }
                                },
                            );
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
            let msg = ClientMessage::PlayerMove {
                left,
                up,
                right,
                down,
            };
            self.client.send(&serialize(&msg).unwrap());
        }

        self.world
            .query::<(&mut Position, &TargetPosition)>(|_, (pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 5.0 * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 5.0 * self.frame_time);
            });
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
