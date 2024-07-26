use std::{net::UdpSocket, time::{Instant, SystemTime}};

use renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};

struct Server {
    renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant
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
            authentication: ServerAuthentication::Unsecure,
        };

        Self {
            renet: RenetServer::new(ConnectionConfig::default()),
            transport: NetcodeServerTransport::new(server_config, socket).unwrap(),
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

    fn handle_events(&mut self) {
        while let Some(event) = self.renet.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }
    }

    fn handle_messages(&mut self, buf: &mut Vec<String>) {
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

fn main() {
    let mut server = Server::new("127.0.0.1:6667");
    let mut buf = Vec::new();

    loop {
        server.update();
        server.handle_events();
        server.handle_messages(&mut buf);
    }
}
