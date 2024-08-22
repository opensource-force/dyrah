use std::collections::HashMap;

use crate::{EntityId, Vec2D};

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub target_pos: Vec2D,
    pub target: EntityId
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

    pub fn spawn_enemy(&mut self) -> Entity {
        self.next_id += 1;
        let id = EntityId::from_raw(self.next_id);

        let enemy = Entity::default();
        self.enemies.insert(id, enemy);

        enemy
    }

    pub fn despawn_entity(&mut self, id: EntityId) {
        self.players.remove(&id);
        self.enemies.remove(&id);
    }
}