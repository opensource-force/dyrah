use std::{collections::HashMap, net::UdpSocket, time::{Instant, SystemTime}};
use dyhra::{ClientChannel, ServerChannel, ServerMessages};
use renet::{transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig}, ClientId, ConnectionConfig, RenetServer, ServerEvent};

struct Server {
    renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant,
}

struct Lobby {
    players: HashMap<ClientId, u64>
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
            last_updated: Instant::now(),
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

                    // spawn player
    
                    //lobby.players.insert(client_id, player_id);

                    let msg = bincode::serialize(
                        &ServerMessages::PlayerCreate {
                            id: client_id.into(),
                            //position:
                        }
                    ).unwrap();
    
                    self.renet.send_message(client_id, ServerChannel::ServerMessages, msg);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Client {} disconnected: {}", client_id, reason);
    
                    //if let Some(player_id) = lobby.players.remove(&client_id) {
                        // delete player
                    //}
    
                    let msg = bincode::serialize(
                        &ServerMessages::PlayerDelete { id: client_id.into() }
                    ).unwrap();
    
                    self.renet.broadcast_message(ServerChannel::ServerMessages, msg);
                }
            }
        }
    
        for client_id in self.renet.clients_id() {
            while let Some(msg) = self.renet.receive_message(
                client_id, ClientChannel::Input
            ) {
                //if let Some(player_id) = lobby.players.get(&client_id) {
                    // handle input
                //}
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