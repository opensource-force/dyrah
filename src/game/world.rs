use std::collections::HashMap;

use crate::{EntityId, Vec2D};

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub target_pos: Vec2D,
    pub target: Option<EntityId>,
    pub health: f32
}

#[derive(Default)]
pub struct World {
    next_id: u64,
    pub players: HashMap<EntityId, Entity>,
    pub enemies: HashMap<EntityId, Entity>
}

impl World {
    pub fn spawn_player(&mut self, id: EntityId) -> &mut Entity {
        let player = Entity::default();

        self.players.insert(id, player);
        self.players.get_mut(&id).unwrap()
    }

    pub fn spawn_enemy(&mut self) -> &mut Entity {
        self.next_id += 1;
        let id = EntityId::from_raw(self.next_id);

        let enemy = Entity::default();
        self.enemies.insert(id, enemy);
        self.enemies.get_mut(&id).unwrap()
    }

    pub fn despawn_entity(&mut self, id: EntityId) {
        self.players.remove(&id);
        self.enemies.remove(&id);
    }
}