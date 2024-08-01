use super::*;
use collections::storage;
use map::{Map, TILE_SIZE};
use std::{net::UdpSocket, sync::mpsc::{Receiver, TryRecvError}, time::{Instant, SystemTime}};

use macroquad::prelude::*;
use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ConnectionConfig, DefaultChannel, RenetClient};
use shipyard::*;
use systems::prelude::*;


pub struct Game(World);

impl Game {
    pub async fn new() -> Self {
        let mut world = World::new();
        storage::store(WorldTime(get_time()));

        let player_tex = load_texture("assets/32rogues/rogues.png").await.unwrap();
        player_tex.set_filter(FilterMode::Nearest);
        let player_id = world.add_entity((
            Position(Vec2::ZERO),
            Velocity(Vec2::ZERO),
            Sprite {
                tex: player_tex,
                frame: ivec2(1, 4),
            },
            Moving(false),
            TargetPosition(Vec2::ZERO),
            Health(100.0),
            Damage(5.0),
        ));

        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();
        monster_tex.set_filter(FilterMode::Nearest);
        let _monster_ids = world.bulk_add_entity((0..199).map(|_| {
            (
                Monster,
                Position(vec2(
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.y),
                )),
                Velocity(Vec2::ZERO),
                Sprite {
                    tex: monster_tex.clone(),
                    frame: ivec2(rand::gen_range(0, 1), rand::gen_range(0, 7)),
                },
                Moving(false),
                TargetPosition(Vec2::ZERO),
                Health(50.0),
            )
        }));

        world.add_unique(Map::new("assets/map.json", "assets/tiles.png").await);
        world.add_unique(Player(player_id));

        world.add_workload(Workloads::events);
        world.add_workload(Workloads::update);
        world.add_workload(Workloads::draw);

        Self(world)
    }

    pub fn events(&self) {
        self.0.run_workload(Workloads::events).unwrap();
    }

    pub fn update(&self) {
        self.0.run_workload(Workloads::update).unwrap();
    }

    pub fn draw(&self) {
        self.0.run_workload(Workloads::draw).unwrap();
    }
}

pub struct Client {
    renet: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant
}

impl Client {
    pub fn new(addr: &str, server_addr: &str) -> Self {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let socket = UdpSocket::bind(addr).unwrap();
        let authentication = ClientAuthentication::Unsecure {
            server_addr: server_addr.parse().unwrap(),
            client_id,
            user_data: None,
            protocol_id: 7,
        };

        Self {
            renet: RenetClient::new(ConnectionConfig::default()),
            transport: NetcodeClientTransport::new(
                current_time,
                authentication,
                socket
            ).unwrap(),
            last_updated: Instant::now()
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();
    }

    pub fn handle_messages(&mut self ) {
        if self.renet.is_connected() {

            while let Some(text) = self.renet.receive_message(
                DefaultChannel::ReliableOrdered
            ) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("Client Received: {}", text);
            }

            self.transport.send_packets(&mut self.renet).unwrap();
        }
    }
}

