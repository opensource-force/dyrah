use std::{
    collections::VecDeque,
    net::UdpSocket,
    time::{Instant, SystemTime}
};

use dyhra::ServerMessages;
use macroquad::prelude::*;
use renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ClientId, ConnectionConfig, RenetClient
};

struct Client {
    renet: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant,
    message_queue: VecDeque<Vec<u8>>
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
            protocol_id: 7
        };

        Self {
            renet: RenetClient::new(ConnectionConfig::default()),
            transport: NetcodeClientTransport::new(
                current_time,
                authentication,
                socket
            ).unwrap(),
            last_updated: Instant::now(),
            message_queue: VecDeque::new(),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();

        while let Some(msg) = self.message_queue.pop_front() {
            let server_msg: ServerMessages = bincode::deserialize(&msg).unwrap();
            
            match server_msg {
                ServerMessages::PlayerCreate { id } => {
                    println!("Player {} connected", ClientId::from(id));
                    // render player
                },
                ServerMessages::PlayerDelete { id } => {
                    println!("Player {} disconnected", ClientId::from(id));
                    // delete player
                }
            }
        }
    }
}

#[macroquad::main("Dyhra")]
async fn main() {
    let mut client = Client::new("127.0.0.1:6668", "127.0.0.1:6667");

    loop {
        clear_background(SKYBLUE);

        client.update();

        next_frame().await;
    }
}