use std::{net::{SocketAddr, UdpSocket}, time::{Instant, SystemTime}};

use dyhra::{ServerChannel, ServerMessages};
use macroquad::{color::SKYBLUE, window::{clear_background, next_frame}};
use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ClientId, ConnectionConfig, RenetClient};

struct Client {
    renet: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant
}

impl Client {
    fn new(server_addr: SocketAddr) -> Self {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id,
            user_data: None,
            protocol_id: 7
        };
    
        Self {
            renet: RenetClient::new(ConnectionConfig::default()),
            transport: NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
            last_updated: Instant::now()
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();

        if self.renet.is_connected() {
            while let Some(msg) = self.renet.receive_message(ServerChannel::ServerMessages) {
                let server_msg: ServerMessages = bincode::deserialize(&msg).unwrap();

                match server_msg {
                    ServerMessages::PlayerCreate { id } => {
                        println!("Player {} joined", ClientId::from(id));
                    },
                    ServerMessages::PlayerDelete { id } => {
                        println!("Player {} left", ClientId::from(id));
                    }
                }
            }
        }

        self.transport.send_packets(&mut self.renet).unwrap();
    }
}

#[macroquad::main("Dyhra")]
async fn main() {
    let mut client = Client::new("127.0.0.1:6667".parse().unwrap());

    loop {
        clear_background(SKYBLUE);

        client.update();

        next_frame().await;
    }
}