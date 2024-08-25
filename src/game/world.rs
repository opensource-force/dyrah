use std::collections::HashMap;

use crate::{EntityId, Sprite, Vec2D};

#[derive(Default, Clone, Copy, PartialEq, Eq)]

pub enum EntityKind {
    Player,
    #[default]
    Enemy
}

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub kind: EntityKind,
    pub sprite: Sprite,
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub target_pos: Option<Vec2D>,
    pub target: Option<EntityId>,
    pub health: f32
}

#[derive(Default)]
pub struct World {
    next_id: u64,
    pub entities: HashMap<EntityId, Entity>
}

impl World {
    pub fn players(&self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.entities.iter().filter(|(_, entity)| entity.kind == EntityKind::Player)
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = (&EntityId, &mut Entity)> {
        self.entities.iter_mut().filter(|(_, entity)| entity.kind == EntityKind::Player)
    }

    pub fn enemies(&self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.entities.iter().filter(|(_, entity)| entity.kind == EntityKind::Enemy)
    }

    pub fn enemies_mut(&mut self) -> impl Iterator<Item = (&EntityId, &mut Entity)> {
        self.entities.iter_mut().filter(|(_, entity)| entity.kind == EntityKind::Enemy)
    }

    pub fn spawn_entity(&mut self, entity: Entity) {
        self.next_id += 1;
        let id = EntityId::from_raw(self.next_id);

        self.entities.insert(id, entity);
    }

    pub fn spawn_entity_from_id(&mut self, id: EntityId, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn despawn_entity(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }
}