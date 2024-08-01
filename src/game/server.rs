use super::*;
use camera::Viewport;
use collections::storage;
use map::{Map, TILE_SIZE};

use std::{net::UdpSocket, time::{Instant, SystemTime}};

use renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use serde_json::json;

pub struct Game(World);

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        storage::store(WorldTime(get_time()));

        let _monster_ids = world.bulk_add_entity((0..199).map(|_| {
            (
                NPC(true),
                Position(vec2(
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.y),
                )),
                Velocity(Vec2::ZERO),
                Moving(false),
                TargetPosition(Vec2::ZERO),
                Health(50.0),
            )
        }));


        world.add_workload(Workloads::events);
        world.add_workload(Workloads::update);

        Self(world)
    }

    pub fn events(&self) {
        self.0.run_workload(Workloads::events).unwrap();
    }

    pub fn update(&self) {
        self.0.run_workload(Workloads::update).unwrap();
    }
}


pub struct Server {
    renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant,
    game: Game
}

impl Server {

    pub fn new(addr: &str) -> Self {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let socket = UdpSocket::bind(addr).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: 7,
            public_addresses: vec![addr.parse().unwrap()],
            authentication: ServerAuthentication::Unsecure,
        };

        Self {
            renet: RenetServer::new(ConnectionConfig::default()),
            transport: NetcodeServerTransport::new(server_config, socket).unwrap(),
            last_updated: Instant::now(),
            game : Game::new()
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.renet.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);
                    let player_ID = self.game.0.add_entity((
                            Position(Vec2::ZERO),
                            Velocity(Vec2::ZERO),
                            Moving(false),
                            Health(100.0),
                            Damage(5.0),
                            NPC(false),
                            Client_ID(client_id)
                    ));
                    self.game.0.run(|npc: View<NPC>|{
                        for (id,NonPC) in npc.iter().with_id(){
                            let mut message = json!({});
                            if NonPC.0{
                                let(velocity,health,position) = self.game.0.borrow::<(View<Velocity>,View<Health>,View<Position>)>().unwrap();
                                message = json!({
                                    "NPC": format!("{}", NonPC),
                                    "id": format!("{:?}",id),
                                    "pos": format!("{}",&position.get(id).unwrap()),
                                    "vel": format!("{}",&velocity.get(id).unwrap()),
                                    "health": format!("{}",&health.get(id).unwrap())
                                });
                            }
                            else{
                                let(velocity,health,position,client_id) = self.game.0.borrow::<(View<Velocity>,View<Health>,View<Position>, View<Client_ID>)>().unwrap();
                                message = json!({
                                    "NPC": format!("{}",NonPC),
                                    "id": format!("{:?}",id),
                                    "pos": format!("{}",&position.get(id).unwrap()),
                                    "vel": format!("{}",&velocity.get(id).unwrap()),
                                    "health": format!("{}",&health.get(id).unwrap()),
                                    "c_id": format!("{}",&client_id.get(id).unwrap())
                                });
                            }
                            println!("{}",message.to_string());
                            let buf = message.to_string().as_bytes().to_vec();
                            self.renet.send_message(client_id,DefaultChannel::ReliableOrdered, buf);
                        }
                    });
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }
    }

    pub fn handle_messages(&mut self, buf: &mut Vec<String>) {
        buf.clear();

        for client_id in self.renet.clients_id() {
            while let Some(msg) = self.renet.receive_message(
                client_id, DefaultChannel::ReliableOrdered
            ) {
                buf.push(String::from_utf8(msg.into()).unwrap());
            }
        }

        for msg in buf.iter() {
            self.renet.broadcast_message(
                DefaultChannel::ReliableOrdered,
                msg.as_bytes().to_vec()
            );
        }
    }
}
