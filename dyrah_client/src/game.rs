use bincode::{deserialize, serialize};
use macroquad::{prelude::*, rand::gen_range, ui::root_ui};
use secs::{Entity, World};
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{
    ClientMessage, Creature, Player, Position, ServerMessage, TargetPosition, map::TILE_SIZE,
};

use crate::{
    CreatureSprite, CreatureTexture, PlayerSprite, PlayerTexture, camera::Camera, map::TiledMap,
};

fn render_system(world: &World) {
    world.query::<(&Creature, &CreatureSprite, &Position)>(|_, (_, creature_spr, pos)| {
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

        draw_rectangle_lines(pos.x, pos.y, TILE_SIZE, TILE_SIZE, 2., RED);
    });

    world.query::<(&Player, &PlayerSprite, &Position, &TargetPosition)>(
        |_, (_, player_spr, pos, target_pos)| {
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

            draw_rectangle_lines(pos.x, pos.y, TILE_SIZE, TILE_SIZE, 2., BLUE);
            draw_rectangle_lines(target_pos.x, target_pos.y, TILE_SIZE, TILE_SIZE, 2., ORANGE);
        },
    );
}

pub struct Game {
    client: Client<Transport>,
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

                    match server_msg {
                        ServerMessage::CreatureSpawned { position } => {
                            self.world.spawn((
                                Creature,
                                CreatureSprite {
                                    frame: (
                                        (gen_range(0, 1) * TILE_SIZE as u32) as f32,
                                        (gen_range(0, 7) * TILE_SIZE as u32) as f32,
                                    ),
                                },
                                position,
                            ));
                        }
                        ServerMessage::CreatureMoved { position } => {
                            self.world
                                .query::<(&Creature, &mut Position)>(|_, (_, pos)| {
                                    pos.x = position.x;
                                    pos.y = position.y;
                                });
                        }
                        ServerMessage::PlayerConnected { position } => {
                            let player = self.world.spawn((
                                Player,
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
                        ServerMessage::PlayerMoved { target_position } => {
                            let mut target_pos = self
                                .world
                                .get_mut::<TargetPosition>(self.player.unwrap())
                                .unwrap();

                            *target_pos = target_position;
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
            .query::<(&Player, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 10. * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 10. * self.frame_time);

                self.camera
                    .attach_sized(pos.x, pos.y, screen_width(), screen_height());
                self.camera.set();
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
