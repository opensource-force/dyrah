use bincode::{deserialize, serialize};
use macroquad::{prelude::*, rand::gen_range, ui::root_ui};
use secs::{Entity, World};
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{
    ClientInput, ClientMessage, Health, Position, ServerMessage, TargetPosition,
    map::{TILE_OFFSET, TILE_SIZE},
};

use crate::{
    Creature, CreatureTexture, Player, PlayerTexture, Sprite, camera::Camera, map::TiledMap,
};

fn render_system(world: &World) {
    let crea_tex = world.get_resource::<CreatureTexture>().unwrap();

    world.query::<(&Creature, &Sprite, &Position, &Health)>(|_, (_, spr, pos, health)| {
        draw_texture_ex(
            &crea_tex.0,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(spr.frame.0, spr.frame.1, TILE_SIZE, TILE_SIZE)),
                ..Default::default()
            },
        );

        draw_rectangle(pos.x, pos.y, health.points / 100.0 * TILE_SIZE, 4., RED);
    });

    let player_tex = world.get_resource::<PlayerTexture>().unwrap();

    world.query::<(&Player, &Position, &TargetPosition, &Health)>(
        |_, (player_state, pos, target_pos, health)| {
            draw_rectangle_lines(target_pos.x, target_pos.y, TILE_SIZE, TILE_SIZE, 2., BLUE);
            draw_circle_lines(
                target_pos.x + TILE_OFFSET,
                target_pos.y + TILE_OFFSET,
                1.,
                2.,
                YELLOW,
            );

            draw_texture_ex(
                &player_tex.0,
                pos.x,
                pos.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        player_state.sprite.frame.0,
                        player_state.sprite.frame.1,
                        TILE_SIZE,
                        TILE_SIZE,
                    )),
                    ..Default::default()
                },
            );

            draw_rectangle(pos.x, pos.y, health.points / 100. * TILE_SIZE, 4., GREEN);
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
    last_input_time: f64,
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
            last_input_time: 0.,
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.client.recv_event() {
            match event {
                ClientEvent::MessageReceived(bytes) => {
                    let msg = deserialize::<ServerMessage>(&bytes).unwrap();

                    self.handle_server_messages(msg);
                }
                _ => {}
            }
        }
    }

    fn handle_server_messages(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::CreatureBatchSpawned(creatures) => {
                for (pos, health) in creatures {
                    self.world.spawn((
                        Creature,
                        Sprite::from_frame(gen_range(0, 1) as f32, gen_range(0, 7) as f32),
                        pos,
                        TargetPosition { x: pos.x, y: pos.y },
                        health,
                    ));
                }
            }
            ServerMessage::CreatureBatchMoved(crea_moves) => {
                for (id, pos) in crea_moves {
                    if let Some(mut target_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                        *target_pos = TargetPosition { x: pos.x, y: pos.y };
                    }
                }
            }
            ServerMessage::PlayerConnected { position, health } => {
                let player = self.world.spawn((
                    Player {
                        sprite: Sprite::from_frame(1., 4.),
                    },
                    position,
                    TargetPosition {
                        x: position.x,
                        y: position.y,
                    },
                    health,
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
                }
            }
        }
    }

    fn update(&mut self) {
        self.client.poll();
        self.handle_events();

        root_ui().label(None, &format!("FPS: {}", get_fps()));

        let current_time = get_time();
        let mouse_world_pos = self.camera.inner.screen_to_world(mouse_position().into());

        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let mut mouse_target_pos = None;

        if is_mouse_button_released(MouseButton::Left) {
            mouse_target_pos = Some(Position {
                x: mouse_world_pos.x,
                y: mouse_world_pos.y,
            })
        }

        if left || up || down || right || mouse_target_pos.is_some() {
            if current_time - self.last_input_time >= 0.2 {
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

                self.last_input_time = current_time;
            }
        }

        self.world
            .query::<(&Player, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 5. * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 5. * self.frame_time);

                self.camera
                    .attach_sized(pos.x, pos.y, screen_width(), screen_height());
                self.camera.set();
            });

        self.world
            .query::<(&Creature, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
                pos.x = pos.x.lerp(target_pos.x, 3. * self.frame_time);
                pos.y = pos.y.lerp(target_pos.y, 3. * self.frame_time);
            });
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLUE);

            self.update();
            self.map.draw_tiles(&self.map_texture);
            self.world.run_systems();

            let mouse_world_pos = self.camera.inner.screen_to_world(mouse_position().into());

            draw_rectangle_lines(
                mouse_world_pos.x,
                mouse_world_pos.y,
                TILE_SIZE,
                TILE_SIZE,
                2.,
                PURPLE,
            );

            next_frame().await;
        }
    }
}
