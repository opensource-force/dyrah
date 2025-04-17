use std::process::exit;

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
    Creature, CreatureTexture, Damage, Damages, Player, PlayerTexture, Sprite,
    camera::Camera,
    map::Map,
    systems::{debug::DebugSystem, movement::MovementSystem, render::RenderSystem},
};

pub struct Game {
    client: Client<Transport>,
    world: World,
    map: Map,
    camera: Camera,
    player: Option<Entity>,
    last_input_time: f64,
    damages: Damages,
}

impl Game {
    pub async fn new() -> Self {
        let transport = Transport::new("127.0.0.1:0");
        let world = World::default();

        set_default_filter_mode(FilterMode::Nearest);
        let player_tex = load_texture("assets/wizard.png").await.unwrap();
        let crea_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        world.add_system(RenderSystem::creature(CreatureTexture(crea_tex)));
        world.add_system(RenderSystem::player(PlayerTexture(player_tex)));
        world.add_system(MovementSystem::creature());

        Self {
            client: Client::new(transport, "127.0.0.1:8080"),
            world,
            map: Map::new("assets/map.json").await,
            camera: Camera::default(),
            player: None,
            last_input_time: 0.,
            damages: Damages::default(),
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
                    if let Some(mut tgt_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                        tgt_pos.vec = pos;
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
                if let Some(mut tgt_pos) = self.world.get_mut::<TargetPosition>(id.into()) {
                    tgt_pos.vec = position;
                    tgt_pos.path = path;
                }
            }
            ServerMessage::EntityDamaged {
                attacker,
                defender,
                hp,
            } => {
                self.world
                    .query(|player: Entity, state: &mut Player, spr: &mut Sprite| {
                        if player.id() == attacker {
                            spr.animation.set_animation(1);
                            state.is_attacking = true;
                        }
                    });

                if let Some(mut health) = self.world.get_mut::<Health>(defender.into()) {
                    let pos = self.world.get::<Position>(defender.into()).unwrap();

                    self.damages.numbers.push(Damage {
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
                    .query(|player: Entity, state: &mut Player, spr: &mut Sprite| {
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
        self.map.draw_tiles();

        RenderSystem::damages(&self.damages)(&self.world);
        MovementSystem::player(&mut self.camera)(&self.world);

        let mouse_screen_pos = mouse_position().into();
        let mouse_world_pos = self.camera.inner.screen_to_world(mouse_screen_pos);
        let mouse_tile_pos = (mouse_world_pos / TILE_SIZE).floor();
        let mouse_tile_world_pos = mouse_tile_pos * TILE_SIZE;

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
                .query(|crea: Entity, _: &Creature, pos: &Position| {
                    if Rect::new(pos.vec.x, pos.vec.y, TILE_SIZE, TILE_SIZE)
                        .contains(mouse_world_pos)
                    {
                        target = Some(crea.id());
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

        self.damages.numbers.retain_mut(|num| {
            num.position.y -= get_frame_time() * 20.;
            num.lifetime -= get_frame_time();
            num.lifetime > 0.
        });

        draw_rectangle_lines(
            mouse_tile_world_pos.x,
            mouse_tile_world_pos.y,
            TILE_SIZE,
            TILE_SIZE,
            2.0,
            ORANGE,
        );

        root_ui().label(None, &format!("FPS: {}", get_fps()));
        root_ui().label(
            None,
            &format!(
                "Mouse: Screen({}, {}) World({}, {}) Tile({}, {})",
                mouse_screen_pos.x,
                mouse_screen_pos.y,
                mouse_world_pos.x,
                mouse_world_pos.y,
                mouse_tile_pos.x,
                mouse_tile_pos.y
            ),
        );

        DebugSystem::player(&self.camera)(&self.world);
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
