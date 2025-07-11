use std::collections::HashMap;

use bincode::{deserialize, serialize};
use egor::{
    app::{Context, InitContext},
    input::{KeyCode, MouseButton},
    math::Vec2,
    render::Color,
};
use secs::{Entity, World};
use wrym::{
    client::{Client, ClientEvent},
    transport::Transport,
};

use dyrah_shared::{
    NetId,
    components::Player,
    messages::{ClientInput, ClientMessage, ServerMessage},
};

use crate::{
    components::{Sprite, TargetWorldPos, WorldPos},
    map::Map,
    sprite::Animation,
};

pub struct Game {
    client: Client<Transport>,
    world: World,
    map: Map,
    lobby: HashMap<NetId, Entity>,
    last_input_time: f32,
    player_tex: Option<usize>,
    player: Option<Entity>,
    player_id: Option<NetId>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            client: Client::new(Transport::new("127.0.0.1:0"), "127.0.0.1:8080"),
            world: World::default(),
            map: Map::new("assets/map.json"),
            lobby: HashMap::new(),
            last_input_time: 0.0,
            player_tex: None,
            player: None,
            player_id: None,
        }
    }

    pub fn load(&mut self, ctx: &mut InitContext) {
        self.map.load(ctx);
        self.player_tex = Some(ctx.load_texture(include_bytes!("../../assets/wizard.png")));
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.client.recv_event() {
            match event {
                ClientEvent::Connected(id) => {
                    println!("Connected to server!");
                    self.player_id = Some(id);
                }
                ClientEvent::Disconnected => {
                    println!("Lost connection to server");
                }
                ClientEvent::MessageReceived(bytes) => {
                    let msg = deserialize::<ServerMessage>(&bytes).unwrap();
                    self.handle_server_messages(msg);
                }
            }
        }
    }

    fn handle_server_messages(&mut self, msg: ServerMessage) {
        match msg {
            ServerMessage::PlayerSpawned { id, position } => {
                println!("Player {} spawned!", id);

                let player = self.world.spawn((
                    Player,
                    WorldPos { vec: position },
                    TargetWorldPos { vec: position },
                    Sprite {
                        anim: Animation::new(1, 6, 6, 0.2),
                        frame_size: Vec2::splat(64.0),
                        sprite_size: Vec2::new(32.0, 64.0),
                    },
                ));

                self.lobby.insert(id, player);
                if self.player_id.is_some() {
                    self.player = Some(player);
                }
            }
            ServerMessage::PlayerMoved { id, position } => {
                println!("Player {} moving..", id);

                if let Some(&player) = self.lobby.get(&id) {
                    let mut target_pos = self.world.get_mut::<TargetWorldPos>(player).unwrap();
                    target_pos.vec = position;
                }
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.client.poll();

        let mouse_pos = ctx.input.mouse_position();
        let left = ctx.input.keys_held(&[KeyCode::KeyA, KeyCode::ArrowLeft]);
        let up = ctx.input.keys_held(&[KeyCode::KeyW, KeyCode::ArrowUp]);
        let right = ctx.input.keys_held(&[KeyCode::KeyD, KeyCode::ArrowRight]);
        let down = ctx.input.keys_held(&[KeyCode::KeyS, KeyCode::ArrowDown]);
        let mouse_tile_pos = ctx
            .input
            .mouse_released(MouseButton::Left)
            .then_some(mouse_pos)
            .map(|mp| self.map.tiled.world_to_tile(mp.into()));
        let moving = left || up || right || down || mouse_tile_pos.is_some();

        self.world.query(
            |_, _: &Player, pos: &mut WorldPos, target_pos: &TargetWorldPos, spr: &mut Sprite| {
                if pos.vec != target_pos.vec {
                    let dir = (target_pos.vec - pos.vec).normalize_or_zero();
                    pos.vec += dir * 100.0 * ctx.timer.delta;

                    if dir.x.abs() > dir.y.abs() {
                        spr.anim.flip_x(dir.x < 0.0);
                    }
                    spr.anim.update(ctx.timer.delta);

                    if pos.vec.distance(target_pos.vec) < 1.0 {
                        pos.vec = target_pos.vec;
                    }
                } else {
                    spr.anim.set_frame(0);
                }
            },
        );

        self.last_input_time += ctx.timer.delta;
        if self.last_input_time >= 0.2 && moving {
            self.last_input_time = 0.0;

            let msg = ClientMessage::PlayerUpdate {
                input: ClientInput {
                    left,
                    up,
                    right,
                    down,
                    mouse_tile_pos,
                },
            };
            self.client.send(&serialize(&msg).unwrap());
        }
    }

    pub fn render(&self, ctx: &mut Context) {
        ctx.graphics.clear(Color::BLUE);

        self.map.draw_tiles(ctx);

        self.world
            .query(|player, _: &Player, world_pos: &WorldPos, spr: &Sprite| {
                let draw_pos = world_pos.vec + spr.anim.offset(spr.frame_size, spr.sprite_size);
                ctx.graphics
                    .rect()
                    .at(draw_pos)
                    .size(Vec2::splat(64.0))
                    .texture(self.player_tex.unwrap())
                    .uv(spr.anim.frame());

                if Some(player) == self.player {
                    ctx.graphics.camera().target(world_pos.vec);
                }
            });
    }
}
