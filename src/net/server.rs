use std::{
   net::{SocketAddr, UdpSocket},
   thread, time::{Duration, Instant, SystemTime}
};

use renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig,},
    ClientId, ConnectionConfig, DisconnectReason, RenetServer, ServerEvent
};
use serde::Serialize;

use crate::{ClientChannel, ClientInput, ClientMessages, ServerChannel};

pub struct Server {
    renet: RenetServer,
    transport: NetcodeServerTransport,
    last_updated: Instant
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
            last_updated: Instant::now()
        }
    }

    pub fn update(&mut self, fps: u64) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();

        self.transport.send_packets(&mut self.renet);

        thread::sleep(Duration::from_millis(1000 / fps));
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

    pub fn send<T: Serialize>(&mut self, id: ClientId, msg: T) {
        let serial_msg = bincode::serialize(&msg).unwrap();

        self.renet.send_message(id, ServerChannel::ServerMessages, serial_msg);
    }

    pub fn broadcast<T: Serialize>(&mut self, msg: T) {
        let serial_msg = bincode::serialize(&msg).unwrap();

        self.renet.broadcast_message(ServerChannel::ServerMessages, serial_msg);
    }
}