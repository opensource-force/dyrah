
use std::{collections::HashMap, net::{SocketAddr, UdpSocket}, time::{Instant, SystemTime}};

use dyhra::{Player, ServerChannel, ServerMessages};
use renet::{transport::{ClientAuthentication, NetcodeClientTransport}, ClientId, ConnectionConfig, RenetClient};

pub struct Client {
    pub renet: RenetClient,
    transport: NetcodeClientTransport,
    last_updated: Instant,
    pub lobby: HashMap<ClientId, Player>
}

impl Client {
    pub fn new(server_addr: SocketAddr) -> (ClientId, Self) {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            server_addr,
            client_id,
            user_data: None,
            protocol_id: 7
        };
    
        (ClientId::from_raw(client_id), Self {
            renet: RenetClient::new(ConnectionConfig::default()),
            transport: NetcodeClientTransport::new(current_time, authentication, socket).unwrap(),
            last_updated: Instant::now(),
            lobby: HashMap::new()
        })
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let duration = now - self.last_updated;
        self.last_updated = now;

        self.renet.update(duration);
        self.transport.update(duration, &mut self.renet).unwrap();
        self.transport.send_packets(&mut self.renet).unwrap();
    }

    pub fn get_server_msg(&mut self) -> Option<ServerMessages> {
            if let Some(msg) = self.renet.receive_message(ServerChannel::ServerMessages) {
                let server_msg = bincode::deserialize(&msg).unwrap();

                return Some(server_msg)
            }

        None
    }
}