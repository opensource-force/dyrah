use std::{net::{SocketAddr, UdpSocket}, time::{Instant, SystemTime}};

use renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};

fn main() {
    let addr: SocketAddr = "127.0.0.1:6667".parse().unwrap();
    let mut server = RenetServer::new(ConnectionConfig::default());
    let socket = UdpSocket::bind(addr).unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: 7,
        public_addresses: vec![addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let mut received_messages = vec![];
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;
        
        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        received_messages.clear();
        
        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Client {} connected", client_id);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
                }
            }
        }
        
        for client_id in server.clients_id() {
            while let Some(msg) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
                let text = String::from_utf8(msg.into()).unwrap();

                println!("Client sent {}", text);

                received_messages.push(text);
            }
        }

        for text in received_messages.iter() {
            server.broadcast_message(DefaultChannel::ReliableOrdered, text.as_bytes().to_vec());
        }
    }
}
