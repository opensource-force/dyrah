use super::*;
use crate::game::{Damage, Dead, Health, Player, Target};

pub struct DamageSystem;

impl DamageSystem {
    pub fn attack_target(
        entities: EntitiesViewMut,
        player: UniqueView<Player>,
        mut health: ViewMut<Health>,
        target: View<Target>,
        damage: View<Damage>,
        mut dead: ViewMut<Dead>,
    ) {
        if let Ok(target) = target.get(player.0) {
            let monster_health = &mut health[target.0];
            monster_health.0 -= damage[player.0].0;

            if monster_health.0 < 0.0 {
                entities.add_component(target.0, &mut dead, Dead(()));
            }
        }
    }
}
