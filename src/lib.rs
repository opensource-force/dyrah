use renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Player {
    pub pos: Position
}


#[derive(Serialize, Deserialize)]
pub enum ClientMessages {
    PlayerCommand {
        id: SerializableClientId
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessages {
    PlayerCreate {
        id: SerializableClientId,
        pos: Position
    },
    PlayerDelete {
        id: SerializableClientId
    },
    PlayerUpdate {
        id: SerializableClientId,
        pos: Position
    }
}


#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool
}


#[derive(Serialize, Deserialize)]
pub enum ClientChannel {
    ClientMessages,
    ClientInput
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities
}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SerializableClientId(u64);


impl From<ClientId> for SerializableClientId {
    fn from(client_id: ClientId) -> Self {
        SerializableClientId(client_id.raw())
    }
}

impl From<SerializableClientId> for ClientId {
    fn from(serializable_id: SerializableClientId) -> Self {
        ClientId::from_raw(serializable_id.0)
    }
}


impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1
        }
    }
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ClientInput => 0,
            ClientChannel::ClientMessages => 1
        }
    }
}