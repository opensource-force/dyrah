use std::collections::HashMap;

use crate::{EntityId, Position};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    #[default]
    Player,
    Enemy
}

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub kind: EntityKind,
    pub pos: Position,
    pub target_pos: Position
}

impl Entity {
    pub fn from_kind(kind: EntityKind) -> Self {
        Self {
            kind,
            pos: Position::default(),
            target_pos: Position::default()
        }
    }
}

#[derive(Default)]
pub struct World {
    pub entities: HashMap<EntityId, Entity>
}

impl World {
    pub fn spawn_entity(&mut self, id: EntityId, kind: EntityKind) -> Entity {
        let entity = Entity::from_kind(kind);

        self.entities.insert(id, entity);

        entity
    }

    pub fn spawn_entity_at(&mut self, id: EntityId, kind: EntityKind, pos: Position) -> Entity {
        let entity = Entity {
            kind,
            pos,
            target_pos: Position::default()
        };

        self.entities.insert(id, entity);

        entity
    }

    pub fn despawn_entity(&mut self, id: EntityId) {
        self.entities.remove(&id);
    }

    pub fn entities_with_kind(&mut self, kind: EntityKind) -> impl Iterator<Item = (&EntityId, &mut Entity)> {
        self.entities.iter_mut().filter(move |(_, entity)| entity.kind == kind)
    }
}