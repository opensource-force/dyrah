mod systems;

use std::{net::UdpSocket, sync::mpsc::{Receiver, TryRecvError}, time::{Instant, SystemTime}};

use dyhra::{spawn_stdin_channel, Player, Position, Sprite, Velocity};
use macroquad::prelude::*;
use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ConnectionConfig, DefaultChannel, RenetClient};
use shipyard::*;
use systems::prelude::*;

struct Client {
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

    fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();
    }

    fn handle_messages(&mut self, channel: &Receiver<String>) {
        if self.renet.is_connected() {
            match channel.try_recv() {
                Ok(text) => {
                    self.renet.send_message(
                        DefaultChannel::ReliableOrdered,
                        text.as_bytes().to_vec()
                    )
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
            }

            while let Some(text) = self.renet.receive_message(
                DefaultChannel::ReliableOrdered
            ) {
                let text = String::from_utf8(text.into()).unwrap();
                println!("{}", text);
            }

            self.transport.send_packets(&mut self.renet).unwrap();
        }
    }
}

#[macroquad::main("Dyhra")]
async fn main() {
    let stdin_channel: Receiver<String> = spawn_stdin_channel();
    let mut client = Client::new("127.0.0.1:6668", "127.0.0.1:6667");

    let mut world = World::new();

    let player_id = world.add_entity((
        Position(vec2(0.0, 0.0)),
        Velocity(vec2(0.0, 0.0)),
        Sprite {
            tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
            frame: ivec2(1, 4)
        }
    ));
    world.add_unique(Player(player_id));

    world.add_workload(Workloads::events);
    world.add_workload(Workloads::update);
    world.add_workload(Workloads::draw);

    loop {
        clear_background(SKYBLUE);

        client.update();

        world.run_workload(Workloads::events).unwrap();
        world.run_workload(Workloads::update).unwrap();
        world.run_workload(Workloads::draw).unwrap();

        client.handle_messages(&stdin_channel);

        next_frame().await;
    }
}