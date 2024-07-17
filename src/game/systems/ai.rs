use super::*;
use collections::storage;
use crate::game::{Player, Position, TargetPosition, Velocity, WorldTime};

pub struct AiSystem;

impl AiSystem {
    pub fn control_monsters(
        player: UniqueView<Player>,
        positions: View<Position>,
        mut velocities: ViewMut<Velocity>,
        mut target_positions: ViewMut<TargetPosition>
    ) {
        if get_time() - storage::get::<WorldTime>().0 > rand::gen_range(1.0, 3.0) {
            for (id, (pos, vel, target_pos)) in (
                &positions, &mut velocities, &mut target_positions
            ).iter().with_id() {
                if id == player.0 {
                    continue
                }

                vel.0 = match rand::gen_range(0, 3) {
                    0 => vec2(0.0, -1.0),
                    1 => vec2(-1.0, 0.0),
                    2 => vec2(0.0, 1.0),
                    3 => vec2(1.0, 0.0),
                    _ => Vec2::ZERO
                };

                if vel.0 != Vec2::ZERO {
                    target_pos.0 = pos.0 + vel.0 * TILE_SIZE;
                }
            }

            storage::store(WorldTime(get_time()));
        }
    }
}