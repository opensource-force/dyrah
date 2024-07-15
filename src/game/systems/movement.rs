use super::*;
use crate::game::{Player, Position, Velocity};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        mut player: UniqueViewMut<Player>,
        mut positions: ViewMut<Position>,
        velocities: View<Velocity>
    ) {
        if player.moving.0 {
            if player.pos.0 == player.target_pos.0 {
                player.moving.0 = false;
            } else {
                let direction = player.target_pos.0 - player.pos.0;
                let dx = direction.x.abs();
                let dy = direction.y.abs();
                let vel = direction.signum();

                if player.pos.0.abs_diff_eq(player.target_pos.0, 1.0) {
                    player.pos.0 = player.target_pos.0;
                } else if dx > dy {
                    player.pos.0.x += vel.x;
                } else if dy > dx {
                    player.pos.0.y += vel.y;
                } else {
                    player.pos.0 += vel;
                }
            }
        }

        for (pos, vel) in (&mut positions, &velocities).iter() {
            pos.0 += vel.0;
        }
    }
}