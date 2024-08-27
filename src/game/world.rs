use std::collections::HashMap;

use thunderdome::{Arena, Index};
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
    pub entities: Arena<Entity>,
    lobby: HashMap<EntityId, Index>
}

impl World {
    pub fn get_entity(&self, entity_id: EntityId) -> Option<&Entity> {
        self.lobby.get(&entity_id).and_then(|&index| self.entities.get(index))
    }

    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<&mut Entity> {
        self.lobby.get(&entity_id).and_then(|&index| self.entities.get_mut(index))
    }
    
    pub fn spawn_entity(&mut self, entity: Entity) {
        self.next_id += 1;
        let entity_id = EntityId(self.next_id);

        let entity_idx = self.entities.insert(entity);
        self.lobby.insert(entity_id, entity_idx);
    }

    pub fn spawn_entity_from_id(&mut self, entity_id: EntityId, entity: Entity) {
        let entity_idx = self.entities.insert(entity);
        self.lobby.insert(entity_id, entity_idx);
    }

    pub fn despawn_entity(&mut self, entity_id: EntityId) {
        if let Some(entity_idx) = self.lobby.remove(&entity_id) {
            self.entities.remove(entity_idx);
        }
    }

    pub fn players(&self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.lobby.iter()
            .filter_map(|(entity_id, &idx)| {
                self.entities.get(idx).and_then(|entity| {
                    if entity.kind == EntityKind::Player {
                        return Some((entity_id, entity))
                    }

                    None
                })
            })
    }

    pub fn enemies(&self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.lobby.iter()
            .filter_map(|(entity_id, &idx)| {
                self.entities.get(idx).and_then(|entity| {
                    if entity.kind == EntityKind::Enemy {
                        return Some((entity_id, entity))
                    }

                    None
                })
            })
    }

    // need entities_mut()/players_mut()/enemies_mut() to be able to mutate enemies from the arena
    // issue with defining functions like above that return mutable references to Entity, is that we borrow self twice, either both mutably or immutably first; causing borrow issues
    // alternatively if we return <EntityId, Index> instead we avoid this issue but the calling side then has to deal with it and that's even arguably even uglier
}