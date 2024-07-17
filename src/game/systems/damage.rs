use super::*;
use crate::game::{Damage, Health, Player, Target};


pub struct DamageSystem;

impl DamageSystem {
    pub fn attack_target(
        player: UniqueView<Player>,
        mut health: ViewMut<Health>,
        target: View<Target>,
        damage: View<Damage>
    ) {
        if let Ok(target) = target.get(player.0) {
            let monster_health = &mut health[target.0];
            monster_health.0 -= damage[player.0].0;
        }
    }
}