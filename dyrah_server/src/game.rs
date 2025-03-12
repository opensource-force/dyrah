use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, Creature, Player, Position, ServerMessage, TargetPosition,
    map::{TILE_SIZE, TiledMap},
};
use rand::{Rng, rng};
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use crate::{Collider, PlayerView};

pub struct Game {
    server: Server<Transport>,
    client_messages: VecDeque<(String, ClientMessage)>,
    lobby: HashMap<String, Entity>,
    world: World,
    map: TiledMap,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");
        let mut world = World::default();
        let map = TiledMap::new("assets/map.json");
        let mut rng = rng();

        world.add_resource(PlayerView::default());

        for _ in 0..50 {
            let pos_x = rng.random_range(0..map.width) as f32 * TILE_SIZE;
            let pos_y = rng.random_range(0..map.height) as f32 * TILE_SIZE;
            world.spawn((
                Creature {
                    last_move: Instant::now(),
                },
                Collider,
                Position { x: pos_x, y: pos_y },
                TargetPosition { x: pos_x, y: pos_y },
            ));
        }

        Self {
            server: Server::new(transport, ServerConfig::default()),
            client_messages: VecDeque::new(),
            lobby: HashMap::new(),
            world,
            map,
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    let mut crea_spawns = Vec::new();

                    self.world.query::<(&Creature, &Position)>(|_, (_, pos)| {
                        crea_spawns.push(*pos);
                    });

                    if !crea_spawns.is_empty() {
                        println!("Spawned {} creatures.", crea_spawns.len());
                        let msg = ServerMessage::CreatureBatchSpawned(crea_spawns);
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), false);
                    }

                    // sync existing players with new clients
                    self.world.query::<(&Player, &Position)>(|_, (_, pos)| {
                        let msg = ServerMessage::PlayerConnected { position: *pos };
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                    });

                    let player_pos = Position {
                        x: self.map.width as f32 / 2.,
                        y: self.map.height as f32 / 2.,
                    };
                    let player = self.world.spawn((
                        Player { moving: false },
                        Collider,
                        player_pos,
                        TargetPosition {
                            x: player_pos.x,
                            y: player_pos.y,
                        },
                    ));
                    let msg = ServerMessage::PlayerConnected {
                        position: player_pos,
                    };

                    self.lobby.insert(addr, player);
                    println!("Player {} spawned.", self.lobby.len());

                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                }
                ServerEvent::ClientDisconnected(_addr) => {
                    unimplemented!()
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let client_msg = deserialize::<ClientMessage>(&bytes).unwrap();

                    self.client_messages.push_back((addr, client_msg));
                }
            }
        }
    }

    fn handle_client_messages(&mut self) {
        while let Some((addr, client_msg)) = self.client_messages.pop_front() {
            let player = self.lobby.get(&addr).unwrap();

            match client_msg {
                ClientMessage::PlayerMove { input } => {
                    let mut player_state = self.world.get_mut::<Player>(*player).unwrap();

                    if player_state.moving {
                        return;
                    }

                    let (dx, dy) = input.to_direction();
                    let pos = self.world.get::<Position>(*player).unwrap();
                    let (tgt_x, tgt_y) = (pos.x + dx * TILE_SIZE, pos.y + dy * TILE_SIZE);

                    if self.is_position_blocked(tgt_x, tgt_y) {
                        return;
                    }

                    if let Some((x, y)) = self.map.get_tile_center("floor", tgt_x, tgt_y) {
                        let mut target_pos = self.world.get_mut::<TargetPosition>(*player).unwrap();

                        player_state.moving = true;
                        (target_pos.x, target_pos.y) = (x, y);
                    }
                }
            }
        }
    }

    fn is_position_blocked(&self, x: f32, y: f32) -> bool {
        let mut blocked = false;

        if !self.map.is_walkable("props", x, y) {
            blocked = true;
            return blocked;
        }

        self.world.query::<(&Collider, &Position)>(|_, (_, pos)| {
            if (pos.x - x).abs() < TILE_SIZE && (pos.y - y).abs() < TILE_SIZE {
                blocked = true;
                return;
            }
        });

        blocked
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
        self.handle_client_messages();

        self.world
            .query::<(&mut Player, &mut Position, &TargetPosition)>(
                |entity, (player, pos, target_pos)| {
                    if player.moving {
                        let speed = TILE_SIZE / 10.;
                        pos.x += (target_pos.x - pos.x).clamp(-speed, speed);
                        pos.y += (target_pos.y - pos.y).clamp(-speed, speed);

                        if (pos.x - target_pos.x).abs() < TILE_SIZE
                            && (pos.y - target_pos.y).abs() < TILE_SIZE
                        {
                            pos.x = target_pos.x;
                            pos.y = target_pos.y;
                            player.moving = false;

                            let mut player_view =
                                self.world.get_resource_mut::<PlayerView>().unwrap();
                            player_view.position = *pos;

                            let msg = ServerMessage::PlayerMoved {
                                id: entity.id(),
                                position: Position {
                                    x: target_pos.x,
                                    y: target_pos.y,
                                },
                            };
                            self.server.broadcast(&serialize(&msg).unwrap())
                        }
                    }
                },
            );

        let mut crea_updates = Vec::new();

        self.world
            .query::<(&mut Creature, &Position, &mut TargetPosition)>(
                |entity, (crea, pos, target_pos)| {
                    let now = Instant::now();
                    if now.duration_since(crea.last_move) >= Duration::from_secs(3) {
                        crea.last_move = now;

                        let mut rng = rng();
                        let (dx, dy) = (rng.random_range(-1..=1), rng.random_range(-1..=1));
                        target_pos.x = pos.x + dx as f32 * TILE_SIZE;
                        target_pos.y = pos.y + dy as f32 * TILE_SIZE;

                        if !self.is_position_blocked(target_pos.x, target_pos.y) {
                            let player_view = self.world.get_resource::<PlayerView>().unwrap();

                            if player_view.contains(target_pos.x, target_pos.y) {
                                crea_updates.push(entity.id());
                            }
                        }
                    }
                },
            );

        let mut crea_moves = Vec::new();

        for id in crea_updates {
            let target_pos = self.world.get::<TargetPosition>(id.into()).unwrap();

            if let Some((x, y)) = self
                .map
                .get_tile_center("floor", target_pos.x, target_pos.y)
            {
                let mut pos = self.world.get_mut::<Position>(id.into()).unwrap();
                let speed = TILE_SIZE / 5.;

                pos.x += (x - pos.x).clamp(-speed, speed);
                pos.y += (y - pos.y).clamp(-speed, speed);

                if (pos.x - target_pos.x).abs() < TILE_SIZE
                    && (pos.y - target_pos.y).abs() < TILE_SIZE
                {
                    pos.x = target_pos.x;
                    pos.y = target_pos.y;
                }
                crea_moves.push((id, *pos));
            }
        }

        if !crea_moves.is_empty() {
            println!("Moving {} creatures..", crea_moves.len());

            let batch_msg = ServerMessage::CreatureBatchMoved(crea_moves);
            self.server.broadcast(&serialize(&batch_msg).unwrap());
        }
    }

    pub fn run(&mut self, fps: u64) {
        let step = Duration::from_secs_f64(1. / fps as f64);
        let mut previous_time = Instant::now();
        let mut lag = Duration::ZERO;

        loop {
            let now = Instant::now();
            let elapsed = now - previous_time;

            previous_time = now;
            lag += elapsed;

            while lag >= step {
                self.update();
                self.world.run_systems();

                lag -= step;
            }

            thread::sleep(step);
        }
    }
}
