mod world;
mod player;
mod enemy;

pub mod prelude {
    pub use super::world::*;
    pub use super::player::*;
    pub use super::enemy::*;
}

use prelude::*;
use macroquad::prelude::*;
use crate::engine::prelude::*;