use std::collections::HashMap;

use crate::{EntityId, Position};

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub pos: Position,
    pub target_pos: Position,
    pub target: EntityId
}

impl Entity {
    pub fn from_pos(pos: Position) -> Self {
        Self {
            pos,
            target_pos: Position::default(),
            target: EntityId::default()
        }
    }
}

#[derive(Default)]
pub struct World {
    next_id: u64,
    pub players: HashMap<EntityId, Entity>,
    pub enemies: HashMap<EntityId, Entity>
}

impl World {
    pub fn spawn_player(&mut self, id: EntityId) -> Entity {
        let player = Entity::default();

        self.players.insert(id, player);

        player
    }

    pub fn spawn_player_at(&mut self, id: EntityId, pos: Position) -> Entity {
        let player = Entity::from_pos(pos);

        self.players.insert(id, player);

        player
    }

    pub fn despawn_entity(&mut self, id: EntityId) {
        self.players.remove(&id);
        self.enemies.remove(&id);
    }

    pub fn spawn_enemy(&mut self) -> Entity {
        self.next_id += 1;
        let id = EntityId::from_raw(self.next_id);

        let enemy = Entity::default();
        self.enemies.insert(id, enemy);

        enemy
    }

    pub fn spawn_enemy_at(&mut self, pos: Position) -> Entity {
        self.next_id += 1;
        let id = EntityId::from_raw(self.next_id);

        let enemy = Entity::from_pos(pos);
        self.enemies.insert(id, enemy);

        enemy
    }
}