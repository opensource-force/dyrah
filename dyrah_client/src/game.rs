use std::{
    process::exit,
    sync::{Arc, RwLock},
};

use bincode::{deserialize, serialize};
use macroquad::{
    prelude::{animation::Animation, *},
    rand::gen_range,
    ui::root_ui,
};
use secs::{Entity, World};
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{
    ClientInput, ClientMessage, Health, Position, ServerMessage, TILE_SIZE, TargetPosition,
};

use crate::{
    Creature, CreatureTexture, Damage, Damages, Player, PlayerTexture, Sprite, camera::Camera,
    map::Map,
};

fn render_system(
    map: Map,
    player_tex: PlayerTexture,
    crea_tex: CreatureTexture,
    damages: Arc<RwLock<Damages>>,
) -> impl Fn(&World) {
    move |world| {
        map.draw_tiles();

        world.query::<(&Creature, &Sprite, &Position, &TargetPosition, &Health)>(
            |_, (_, spr, pos, target_pos, health)| {
                draw_rectangle_lines(
                    target_pos.vec.x,
                    target_pos.vec.y,
                    TILE_SIZE,
                    TILE_SIZE,
                    2.,
                    GRAY,
                );
                draw_texture_ex(
                    &crea_tex.0,
                    pos.vec.x,
                    pos.vec.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(spr.frame.0, spr.frame.1, TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                );

                draw_rectangle(
                    pos.vec.x,
                    pos.vec.y,
                    health.points / 100. * TILE_SIZE,
                    4.,
                    RED,
                );
            },
        );

        world.query::<(
            &mut Player,
            &mut Sprite,
            &Position,
            &TargetPosition,
            &Health,
        )>(|_, (_, spr, pos, target_pos, health)| {
            if let Some(path) = &target_pos.path {
                for window in path.windows(2) {
                    let start = window[0];
                    let end = window[1];
                    draw_line(start.x, start.y, end.x, end.y, 3.0, ORANGE);
                }

                for point in path {
                    draw_circle_lines(point.x, point.y, 4.0, 2.0, BLACK);
                }
            }

            draw_rectangle_lines(
                target_pos.vec.x,
                target_pos.vec.y,
                TILE_SIZE,
                TILE_SIZE,
                2.,
                WHITE,
            );

            draw_texture_ex(
                &player_tex.0,
                pos.vec.x - spr.is_flipped.x as i8 as f32 * TILE_SIZE,
                pos.vec.y - TILE_SIZE,
                WHITE,
                DrawTextureParams {
                    source: Some(spr.animation.frame().source_rect),
                    dest_size: Some(spr.animation.frame().dest_size),
                    flip_x: spr.is_flipped.x,
                    flip_y: spr.is_flipped.y,
                    ..Default::default()
                },
            );

            draw_rectangle(
                pos.vec.x,
                pos.vec.y - TILE_SIZE,
                health.points / 100. * TILE_SIZE,
                4.,
                GREEN,
            );
        });

        for num in &damages.read().unwrap().numbers {
            if let Some(pos) = world.get::<Position>(num.origin.into()) {
                draw_rectangle_lines(pos.vec.x, pos.vec.y, TILE_SIZE, TILE_SIZE, 2., BLACK);

                draw_text(
                    &num.value.to_string(),
                    num.position.x,
                    num.position.y,
                    16.,
                    RED,
                );
            }
        }
    }
}

fn movement_system(cam: Arc<RwLock<Camera>>) -> impl Fn(&World) {
    move |world| {
        let frame_time = get_frame_time();

        world.query::<(&Player, &mut Sprite, &mut Position, &TargetPosition)>(
            |_, (_, spr, pos, target_pos)| {
                pos.vec = pos.vec.lerp(target_pos.vec, 5. * frame_time);

                if target_pos.vec.x < pos.vec.x {
                    spr.is_flipped.x = true;
                } else if target_pos.vec.x > pos.vec.x {
                    spr.is_flipped.x = false;
                }

                spr.animation.update();
                let mut cam = cam.write().unwrap();
                cam.attach_sized(pos.vec.x, pos.vec.y, screen_width(), screen_height());
                cam.set();
            },
        );

        world.query::<(&Creature, &mut Position, &TargetPosition)>(|_, (_, pos, target_pos)| {
            pos.vec = pos.vec.lerp(target_pos.vec, 3. * frame_time);
        });
    }
}

pub struct Game {
    client: Client<Transport>,
    world: World,
    player: Option<Entity>,
    last_input_time: f64,
    damages: Arc<RwLock<Damages>>,
    camera: Arc<RwLock<Camera>>,
}

impl Game {
    pub async fn new() -> Self {
        let transport = Transport::new("127.0.0.1:0");
        let world = World::default();

        set_default_filter_mode(FilterMode::Nearest);
        let player_tex = load_texture("assets/wizard.png").await.unwrap();
        let monsters_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        let map = Map::new("assets/map.json").await;
        let camera: Arc<RwLock<Camera>> = Arc::new(RwLock::new(Camera::default()));
        let player_texture = PlayerTexture(player_tex);
        let creature_texture = CreatureTexture(monsters_tex);
        let damages = Arc::new(RwLock::new(Damages::default()));

        world.add_system(render_system(
            map,
            player_texture,
            creature_texture,
            damages.clone(),
        ));
        world.add_system(movement_system(camera.clone()));

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            world,
            damages,
            camera,
            player: None,
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
                for (pos, hp) in creatures {
                    self.world.spawn((
                        Creature,
                        Sprite::from_frame(gen_range(0, 1) as f32, gen_range(0, 7) as f32),
                        Position::new(pos),
                        TargetPosition::new(pos),
                        Health { points: hp },
                    ));
                }
            }
            ServerMessage::CreatureBatchMoved(crea_moves) => {
                for (id, pos) in crea_moves {
                    if let Some(mut target_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                        target_pos.vec = pos;
                    }
                }
            }
            ServerMessage::PlayerConnected { position, hp } => {
                let player_anims = &[
                    Animation {
                        name: "idle".to_string(),
                        row: 0,
                        frames: 1,
                        fps: 0,
                    },
                    Animation {
                        name: "casting".to_string(),
                        row: 0,
                        frames: 6,
                        fps: 5,
                    },
                ];
                let player = self.world.spawn((
                    Player {
                        is_attacking: false,
                    },
                    Sprite::new(player_anims),
                    Position::new(position),
                    TargetPosition::new(position),
                    Health { points: hp },
                ));

                if self.player.is_none() {
                    self.player = Some(player);
                }
            }
            ServerMessage::PlayerMoved { id, position, path } => {
                if let Some(mut target_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                    target_pos.vec = position;
                    target_pos.path = path;
                }
            }
            ServerMessage::EntityDamaged {
                attacker,
                defender,
                hp,
            } => {
                self.world
                    .query::<(&mut Player, &mut Sprite)>(|player, (state, spr)| {
                        if player.id() == attacker {
                            spr.animation.set_animation(1);
                            state.is_attacking = true;
                        }
                    });

                if let Some(mut health) = self.world.get_mut::<Health>(defender.into()) {
                    let mut damages = self.damages.write().unwrap();
                    let pos = self.world.get::<Position>(defender.into()).unwrap();

                    damages.numbers.push(Damage {
                        origin: attacker,
                        value: health.points - hp,
                        position: vec2(pos.vec.x, pos.vec.y + 2.),
                        lifetime: 1.,
                    });

                    health.points = hp;
                }
            }
            ServerMessage::EntityDied { killer, victim } => {
                self.world.despawn(victim.into());

                if self.player == Some(victim.into()) {
                    exit(1);
                }

                self.world
                    .query::<(&mut Player, &mut Sprite)>(|player, (state, spr)| {
                        if player.id() == killer {
                            spr.animation.set_animation(0);
                            state.is_attacking = false;
                        }
                    });
            }
        }
    }

    fn update(&mut self) {
        self.client.poll();
        self.handle_events();

        root_ui().label(None, &format!("FPS: {}", get_fps()));

        let mouse_world_pos = self
            .camera
            .read()
            .unwrap()
            .inner
            .screen_to_world(mouse_position().into());
        let current_time = get_time();

        let left = is_key_down(KeyCode::A) || is_key_down(KeyCode::Left);
        let up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Up);
        let right = is_key_down(KeyCode::D) || is_key_down(KeyCode::Right);
        let down = is_key_down(KeyCode::S) || is_key_down(KeyCode::Down);
        let mouse_target_pos = if is_mouse_button_released(MouseButton::Left) {
            Some(mouse_world_pos)
        } else {
            None
        };
        let mouse_target = if is_mouse_button_released(MouseButton::Right) {
            let mut target = None;

            self.world
                .query::<(&Creature, &Position)>(|entity, (_, pos)| {
                    if Rect::new(pos.vec.x, pos.vec.y, TILE_SIZE, TILE_SIZE)
                        .contains(mouse_world_pos)
                    {
                        target = Some(entity.id());
                        return;
                    }
                });

            target
        } else {
            None
        };

        if left || up || down || right || mouse_target_pos.is_some() || mouse_target.is_some() {
            if current_time - self.last_input_time >= 0.2 {
                self.last_input_time = current_time;

                let msg = ClientMessage::PlayerUpdate {
                    input: ClientInput {
                        left,
                        up,
                        right,
                        down,
                        mouse_target_pos,
                        mouse_target,
                    },
                };
                self.client.send(&serialize(&msg).unwrap());
            }
        }

        self.damages.write().unwrap().numbers.retain_mut(|num| {
            num.position.y -= get_frame_time() * 20.;
            num.lifetime -= get_frame_time();
            num.lifetime > 0.
        });
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(BLUE);

            self.update();
            self.world.run_systems();

            next_frame().await;
        }
    }
}
