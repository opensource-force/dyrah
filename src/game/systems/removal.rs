use crate::game::Dead;

use super::*;
pub struct RemovalSystem;

impl RemovalSystem {
    pub fn remove_dead (
        mut world: AllStoragesViewMut,
    ) {
        world.delete_any::<SparseSet<Dead>>();
    }


}
