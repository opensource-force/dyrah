use std::{collections::VecDeque, time::Instant};

use bincode::{deserialize, serialize};
use macroquad::{prelude::*, rand::gen_range, ui::root_ui};
use secs::{Entity, World};
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{
    ClientInput, ClientMessage, Creature, Player, Position, ServerMessage, TargetPosition,
    map::{TILE_OFFSET, TILE_SIZE},
};

use crate::{
    CreatureSprite, CreatureTexture, PlayerSprite, PlayerTexture, camera::Camera, map::TiledMap,
};

fn render_system(world: &World) {
    world.query::<(&Creature, &CreatureSprite, &Position, &TargetPosition)>(
        |_, (_, creature_spr, pos, target_pos)| {
            draw_rectangle_lines(target_pos.x, target_pos.y, TILE_SIZE, TILE_SIZE, 2., YELLOW);
            draw_rectangle_lines(pos.x, pos.y, TILE_SIZE, TILE_SIZE, 2., RED);

            let creature_tex = world.get_resource::<CreatureTexture>().unwrap();

            draw_texture_ex(
                &creature_tex.0,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        creature_spr.frame.0,
                        creature_spr.frame.1,
                        TILE_SIZE,
                        TILE_SIZE,
                    )),
                    ..Default::default()
                },
            );
        },
    );

    world.query::<(&Player, &PlayerSprite, &Position, &TargetPosition)>(
        |_, (_, player_spr, pos, target_pos)| {
            draw_rectangle_lines(target_pos.x, target_pos.y, TILE_SIZE, TILE_SIZE, 2., YELLOW);
            draw_circle_lines(
                target_pos.x + TILE_OFFSET,
                target_pos.y + TILE_OFFSET,
                1.,
                2.,
                GREEN,
            );
            draw_rectangle_lines(pos.x, pos.y, TILE_SIZE, TILE_SIZE, 2., BLUE);

            let player_tex = world.get_resource::<PlayerTexture>().unwrap();

            draw_texture_ex(
                &player_tex.0,
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
        },
    );
}

pub struct Game {
    client: Client<Transport>,
    server_messages: VecDeque<ServerMessage>,
    world: World,
    map: TiledMap,
    map_texture: Texture2D,
    camera: Camera,
    player: Option<Entity>,
    frame_time: f32,
}

impl Game {
    pub async fn new() -> Self {
        let transport = Transport::new("127.0.0.1:0");
        let mut world = World::default();
        let map = TiledMap::new("assets/map.json");
        let rogues_tex = load_texture("assets/32rogues/rogues.png").await.unwrap();
        let monsters_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        set_default_filter_mode(FilterMode::Nearest);

        world.add_resource(PlayerTexture(rogues_tex));
        world.add_resource(CreatureTexture(monsters_tex));

        world.add_system(render_system);

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            server_messages: VecDeque::new(),
            world,
            map,
            map_texture: load_texture("assets/tiles.png").await.unwrap(),
            camera: Camera::default(),
            player: None,
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

                    self.server_messages.push_back(server_msg);
                }
            }
        }
    }

    fn handle_server_messages(&mut self) {
        while let Some(server_msg) = self.server_messages.pop_front() {
            match server_msg {
                ServerMessage::CreatureBatchSpawned(positions) => {
                    for pos in positions {
                        self.world.spawn((
                            Creature {
                                last_move: Instant::now(),
                            },
                            CreatureSprite {
                                frame: (
                                    (gen_range(0, 1) * TILE_SIZE as u32) as f32,
                                    (gen_range(0, 7) * TILE_SIZE as u32) as f32,
                                ),
                            },
                            pos,
                            TargetPosition { x: pos.x, y: pos.y },
                        ));
                    }
                }
                ServerMessage::CreatureBatchMoved(crea_moves) => {
                    for (id, pos) in crea_moves {
                        if let Some(mut target_pos) =
                            self.world.get_mut::<TargetPosition>(id.into())
                        {
                            *target_pos = TargetPosition { x: pos.x, y: pos.y };
                        }
                    }
                }
                ServerMessage::PlayerConnected { position } => {
                    let player = self.world.spawn((
                        Player { moving: false },
                        PlayerSprite { frame: (0., 0.) },
                        position,
                        TargetPosition {
                            x: position.x,
                            y: position.y,
                        },
                    ));

                    if self.player.is_none() {
                        self.player = Some(player);
                    }
                }
                ServerMessage::PlayerMoved { id, position } => {
                    if let Some(mut target_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                        *target_pos = TargetPosition {
                            x: position.x,
                            y: position.y,
                        };

                        // need to keep target_position attached to player position (fixes crazy camera)
                        self.camera.attach_sized(
                            target_pos.x,
                            target_pos.y,
                            screen_width(),
                            screen_height(),
                        );
                        self.camera.set();
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        self.client.poll();
        self.handle_events();
        self.handle_server_messages();

        root_ui().label(None, &format!("FPS: {}", get_fps()));

        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let mouse_world_pos = self.camera.inner.screen_to_world(mouse_position().into());
        let mut mouse_target_pos = None;

        if is_mouse_button_released(MouseButton::Left) {
            mouse_target_pos = Some(Position {
                x: mouse_world_pos.x,
                y: mouse_world_pos.y,
            })
        }

        if left || up || down || right || mouse_target_pos.is_some() {
            let msg = ClientMessage::PlayerMove {
                input: ClientInput {
                    left,
                    up,
                    right,
                    down,
                    mouse_target_pos,
                },
            };
            self.client.send(&serialize(&msg).unwrap());
        }

        self.world
            .query::<(&Player, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 5. * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 5. * self.frame_time);
            });

        self.world
            .query::<(&Creature, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 3. * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 3. * self.frame_time);
            });
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(SKYBLUE);

            self.update();
            self.map.draw_tiles(&self.map_texture);
            self.world.run_systems();

            let mouse_world_pos = self.camera.inner.screen_to_world(mouse_position().into());

            draw_rectangle_lines(
                mouse_world_pos.x - TILE_OFFSET,
                mouse_world_pos.y - TILE_OFFSET,
                TILE_SIZE,
                TILE_SIZE,
                2.,
                PURPLE,
            );

            next_frame().await;
        }
    }
}
