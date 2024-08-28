pub mod net;
pub mod game;

use game::{EntityId, Vec2D};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub mouse_target_pos: Option<Vec2D>,
    pub mouse_target: Option<EntityId>
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessages {
    PlayerAttack {
        id: EntityId,
        enemy_id: EntityId
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessages {
    PlayerCreate {
        id: EntityId,
        pos: Vec2D,
        health: f32
    },
    PlayerDelete {
        id: EntityId
    },
    PlayerUpdate {
        id: EntityId,
        pos: Vec2D,
        target: Option<EntityId>
    },
    EnemyCreate {
        id: EntityId,
        pos: Vec2D,
        health: f32
    },
    EnemyDelete {
        id: EntityId
    },
    EnemyUpdate {
        id: EntityId,
        health: f32
    }
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

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::ClientInput => 0,
            ClientChannel::ClientMessages => 1
        }
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