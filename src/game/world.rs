use thunderdome::{Arena, Index};

use super::{EntityId, Sprite, Vec2D};

#[derive(Default, Clone, Copy)]
pub struct Entity {
    pub sprite: Sprite,
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub target_pos: Option<Vec2D>,
    pub target: Option<EntityId>,
    pub health: f32,
    pub damage: f32
}

#[derive(Default)]
pub struct World {
    pub entities: Arena<Entity>
}

impl World {
    pub fn spawn_entity(&mut self, entity: Entity) -> Index {
        self.entities.insert(entity)
    }

    pub fn despawn_entity(&mut self, idx: Index) -> Option<Entity> {
        self.entities.remove(idx)
    }
}