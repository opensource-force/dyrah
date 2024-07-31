use std::{
    collections::VecDeque,
    net::UdpSocket,
    time::{Instant, SystemTime}
};

use dyhra::{ClientChannel, ServerMessages};
use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ClientId, ConnectionConfig, RenetServer, ServerEvent
};

struct Server {
    renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant,
    message_queue: VecDeque<(ClientId, Vec<u8>)>
}

impl Server {
    fn new(addr: &str) -> Self {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let socket = UdpSocket::bind(addr).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: 7,
            public_addresses: vec![addr.parse().unwrap()],
            authentication: ServerAuthentication::Unsecure
        };

        Self {
            renet: RenetServer::new(ConnectionConfig::default()),
            transport: NetcodeServerTransport::new(server_config, socket).unwrap(),
            last_updated: Instant::now(),
            message_queue:VecDeque::new()
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();
        
        while let Some(event) = self.renet.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);

                    let msg = bincode::serialize(
                        &ServerMessages::PlayerCreate { id: client_id.into() }
                    ).unwrap();

                    self.message_queue.push_back((client_id, msg));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);

                    let msg = bincode::serialize(
                        &ServerMessages::PlayerDelete { id: client_id.into() }
                    ).unwrap();

                    self.message_queue.push_back((client_id, msg));
                }
            }
        }

        for client_id in self.renet.clients_id() {
            while let Some(msg) = self.renet.receive_message(
                client_id, ClientChannel::Input
            ) {
                /*
                if let Some(player_id) = lobby.players.get(&client_id) {
                    // handle input
                }
                */
    
                //self.message_queue.push_back(msg.to_vec());
            }
        }

    }
}

fn main() {
    let mut server = Server::new("127.0.0.1:6667");

    loop {
        server.update();
    }
}