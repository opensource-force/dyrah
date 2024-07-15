use super::*;
use collections::storage;
use crate::game::{Velocity, WorldTime};

pub struct AiSystem;

impl AiSystem {
    pub fn control_monsters(mut velocities: ViewMut<Velocity>) {
        if get_time() - storage::get::<WorldTime>().0 > rand::gen_range(1.0, 3.0) {
            for vel in (&mut velocities).iter() {
                vel.0 = match rand::gen_range(0, 3) {
                    0 => vec2(0.0, -1.0),
                    1 => vec2(-1.0, 0.0),
                    2 => vec2(0.0, 1.0),
                    3 => vec2(1.0, 0.0),
                    _ => Vec2::ZERO
                };
            }

            storage::store(WorldTime(get_time()));
        }
    }
}