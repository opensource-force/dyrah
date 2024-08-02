use std::{
    collections::{HashMap, VecDeque},
    net::{SocketAddr, UdpSocket},
    time::{Instant, SystemTime},
};

use dyhra::{ClientChannel, ClientInput, ClientMessages, Player, ServerChannel};
use renet::{
    transport::{
        NetcodeServerTransport, ServerAuthentication, ServerConfig,
    }, ClientId, ConnectionConfig, DisconnectReason, RenetServer, ServerEvent
};

pub struct Server {
    pub renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant,
    pub message_queue: VecDeque<Vec<u8>>,
    pub lobby: HashMap<ClientId, Player>
}

impl Server {
    pub fn new(public_addr: SocketAddr) -> Self {
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: 7,
            public_addresses: vec![public_addr],
            authentication: ServerAuthentication::Unsecure,
        };
        let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

        Self {
            renet: RenetServer::new(ConnectionConfig::default()),
            transport: NetcodeServerTransport::new(server_config, socket).unwrap(),
            last_updated: Instant::now(),
            message_queue: VecDeque::new(),
            lobby: HashMap::new()
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();

        self.transport.send_packets(&mut self.renet);

        if let Some(msg) = self.message_queue.pop_back() {
            self.renet.broadcast_message(ServerChannel::ServerMessages, msg);
        }
    }

    pub fn on_client_connect(&mut self) -> Option<ClientId> {
        while let Some(event) = self.renet.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    return Some(client_id)
                }
                _ => return None
            }
        }

        None
    } 

    pub fn on_client_disconnect(&mut self) -> Option<(ClientId, DisconnectReason)> {
        while let Some(event) = self.renet.get_event() {
            match event {
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    return Some((client_id, reason))
                }
                _ => return None
            }
        }

        None
    }

    pub fn get_client_msg(&mut self) -> Option<(ClientId, ClientMessages)> {
        for client_id in self.renet.clients_id() {
            if let Some(msg) = self.renet.receive_message(client_id, ClientChannel::ClientMessages) {
                let client_msg = bincode::deserialize(&msg).unwrap();

                return Some((client_id, client_msg));
            }
        }
        
        None
    }

    pub fn get_client_input(&mut self) -> Option<(ClientId, ClientInput)> {
        for client_id in self.renet.clients_id() {
            if let Some(msg) = self.renet.receive_message(client_id, ClientChannel::ClientInput) {
                let client_input = bincode::deserialize(&msg).unwrap();

                return Some((client_id, client_input))
            }
        }

        None
    }
}