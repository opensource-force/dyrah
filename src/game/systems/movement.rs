use super::*;
use crate::game::{Player, Position, Velocity};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        mut player: UniqueViewMut<Player>,
        mut positions: ViewMut<Position>,
        velocities: View<Velocity>
    ) {
        let vel = player.vel.0;
        player.pos.0 += vel;

        for (pos, vel) in (&mut positions, &velocities).iter() {
            pos.0 += vel.0;
        }
    }
}