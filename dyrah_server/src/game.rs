use std::{
    collections::HashMap,
    thread,
    time::{Duration, Instant},
};

use bincode::{deserialize, serialize};
use dyrah_shared::{
    ClientMessage, GameEvent, Health, Position, ServerMessage, TILE_SIZE, TargetPosition, vec2,
};
use rand::{Rng, rng};
use secs::{Entity, World};
use wrym::{
    server::{Server, ServerConfig, ServerEvent},
    transport::Transport,
};

use crate::{
    Collider, Creature, Player, PlayerView, State,
    map::{CollisionGrid, Map},
    systems::{combat::CombatSystem, input::InputSystem, movement::MovementSystem},
};

pub struct Game {
    events: Vec<GameEvent>,
    server: Server<Transport>,
    lobby: HashMap<String, Entity>,
    world: World,
    map: Map,
    grid: CollisionGrid,
    player_view: PlayerView,
    dead_entities: Vec<(u64, u64)>,
}

impl Game {
    pub fn new() -> Self {
        let transport = Transport::new("127.0.0.1:8080");
        let world = World::default();
        let map = Map::new("assets/map.json");
        let grid = CollisionGrid::new(&map);
        let player_view = PlayerView::new(map.get_spawn("player").unwrap());
        let mut rng = rng();

        for _ in 0..299 {
            let pos = vec2(
                rng.random_range(0..map.tiled.width) as f32 * TILE_SIZE,
                rng.random_range(0..map.tiled.height) as f32 * TILE_SIZE,
            );

            world.spawn((
                Creature,
                State::new(),
                Collider,
                Position::new(pos),
                TargetPosition::new(pos),
                Health {
                    points: rng.random_range(50.0..80.),
                },
            ));
        }

        Self {
            events: Vec::new(),
            server: Server::new(transport, ServerConfig::default()),
            lobby: HashMap::new(),
            world,
            map,
            grid,
            player_view,
            dead_entities: Vec::new(),
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.server.recv_event() {
            match event {
                ServerEvent::ClientConnected(addr) => {
                    println!("Client connected from {}", addr);

                    let mut crea_spawns = Vec::new();

                    self.world
                        .query(|_, _: &Creature, pos: &Position, health: &Health| {
                            crea_spawns.push((pos.vec, health.points));
                        });

                    if !crea_spawns.is_empty() {
                        println!("Spawned {} creatures.", crea_spawns.len());

                        let msg = ServerMessage::CreatureBatchSpawned(crea_spawns);
                        self.server
                            .send_reliable_to(&addr, &serialize(&msg).unwrap(), false);
                    }

                    self.world
                        .query(|_, _: &Player, pos: &Position, health: &Health| {
                            let msg = ServerMessage::PlayerConnected {
                                position: pos.vec,
                                hp: health.points,
                            };
                            self.server
                                .send_reliable_to(&addr, &serialize(&msg).unwrap(), true);
                        });

                    let spawn_pos = self.map.get_spawn("player").unwrap();
                    let player_health = Health { points: 100. };
                    let player = self.world.spawn((
                        Player,
                        State::new(),
                        Collider,
                        Position::new(spawn_pos),
                        TargetPosition::new(spawn_pos),
                        player_health,
                    ));

                    self.lobby.insert(addr, player);

                    println!("Player {} spawned.", self.lobby.len());

                    let msg = ServerMessage::PlayerConnected {
                        position: spawn_pos,
                        hp: player_health.points,
                    };
                    self.server
                        .broadcast_reliable(&serialize(&msg).unwrap(), true);
                }
                ServerEvent::ClientDisconnected(addr) => {
                    println!("Client {} disconnected.", addr);
                }
                ServerEvent::MessageReceived(addr, bytes) => {
                    let msg = deserialize::<ClientMessage>(&bytes).unwrap();
                    self.handle_client_messages(&addr, msg);
                }
            }
        }
    }

    fn handle_client_messages(&mut self, addr: &str, msg: ClientMessage) {
        match msg {
            ClientMessage::PlayerUpdate { input } => {
                if let Some(player) = self.lobby.get(addr) {
                    InputSystem::player(*player, input, &self.map, &self.grid)(&self.world);
                }
            }
        }
    }

    fn update(&mut self) {
        self.server.poll();
        self.handle_events();
        self.grid.update(&self.map, &self.world);

        MovementSystem::player(
            &mut self.events,
            &self.map,
            &mut self.player_view,
            &self.grid,
        )(&self.world);
        CombatSystem::player(&mut self.events, &mut self.dead_entities)(&self.world);
        MovementSystem::creature(&mut self.events, &self.map, &self.grid, &self.player_view)(
            &self.world,
        );
        CombatSystem::creature(&mut self.events, &mut self.lobby, &mut self.dead_entities)(
            &self.world,
        );

        for event in self.events.drain(..) {
            match event {
                GameEvent::PlayerMoved { id, position, path } => {
                    self.server.broadcast(
                        &serialize(&ServerMessage::PlayerMoved { id, position, path }).unwrap(),
                    );
                }
                GameEvent::CreatureBatchMoved(batch) => {
                    self.server
                        .broadcast(&serialize(&ServerMessage::CreatureBatchMoved(batch)).unwrap());
                }
                GameEvent::EntityDamaged {
                    attacker,
                    defender,
                    hp,
                } => {
                    self.server.broadcast_reliable(
                        &serialize(&ServerMessage::EntityDamaged {
                            attacker,
                            defender,
                            hp,
                        })
                        .unwrap(),
                        false,
                    );
                }
                GameEvent::EntityDied { killer, victim } => {
                    self.server.broadcast_reliable(
                        &serialize(&ServerMessage::EntityDied { killer, victim }).unwrap(),
                        false,
                    );
                }
            }
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
