use super::*;
use crate::game::{Damage, Dead, Health, Player, Target};

pub struct DamageSystem;

impl DamageSystem {
    pub fn attack_target(
        entities: EntitiesView,
        player: UniqueView<Player>,
        mut healths: ViewMut<Health>,
        mut targets: ViewMut<Target>,
        damages: View<Damage>,
        mut deads: ViewMut<Dead>
    ) {
        if let Ok(monster) = targets.get(player.0) {
            let monster_hp = &mut healths[monster.0];
            monster_hp.0 -= damages[player.0].0;

            if monster_hp.0 < 0.0 {
                entities.add_component(monster.0, &mut deads, Dead);
                targets.remove(player.0);
            }
        }
    }
}
