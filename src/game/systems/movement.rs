use super::*;
use crate::game::{Moving, Player, Position, TargetPosition, Velocity};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        player: UniqueView<Player>,
        mut positions: ViewMut<Position>,
        velocities: View<Velocity>,
        mut moving: ViewMut<Moving>,
        target_positions: View<TargetPosition>,
    ) {
        let player_pos = &mut positions[player.0];
        let player_target_pos = &target_positions[player.0];

        if moving[player.0].0 {
            if player_pos.0 == player_target_pos.0 {
                moving[player.0].0 = false;
            } else {
                let direction = player_target_pos.0 - player_pos.0;
                let dx = direction.x.abs();
                let dy = direction.y.abs();
                let vel = direction.signum();

                if player_pos.0.abs_diff_eq(player_target_pos.0, 1.0) {
                    player_pos.0 = player_target_pos.0;
                } else if dx > dy {
                    player_pos.0.x += vel.x;
                } else if dy > dx {
                    player_pos.0.y += vel.y;
                } else {
                    player_pos.0 += vel;
                }
            }
        }

        for (pos, vel) in (&mut positions, &velocities).iter() {
            pos.0 += vel.0;
        }
    }
}
