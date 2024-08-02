use renet::ClientId;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessages {
    PlayerCreate {
        id: SerializableClientId
    },
    PlayerDelete {
        id: SerializableClientId
    }
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

pub enum ClientChannel {
    Input,
    Command,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Velocity {
    x: f32,
    y: f32
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInput {
    pub velocity: Velocity
}