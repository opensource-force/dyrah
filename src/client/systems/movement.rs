use shipyard::{UniqueView, View, ViewMut};

use crate::{Player, Position, Velocity};

pub struct MovementSystem;

impl MovementSystem {
    pub fn update(
        player: UniqueView<Player>,
        mut positions: ViewMut<Position>,
        velocities: View<Velocity>
    ) {
        let pos = &mut positions[player.0];
        let vel = &velocities[player.0];

        pos.0 += vel.0;
    }
}