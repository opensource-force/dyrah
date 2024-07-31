use std::{sync::mpsc::{self, Receiver}, thread};

use renet::ClientId;
use serde::{Deserialize, Serialize};

pub struct Player(u64);

pub struct Position {
    pub x: f32,
    pub y: f32
}


pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();

        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer.trim_end().to_string()).unwrap();
    });
    rx
}

#[derive(Serialize, Deserialize)]
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

pub enum ClientChannel {
    Input,
    Command,
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessages {
    PlayerCreate {
        id: SerializableClientId
    },
    PlayerDelete {
        id: SerializableClientId
    }
}