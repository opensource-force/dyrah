mod render;
mod input;
mod movement;
mod ai;

pub mod prelude {
    pub use super::render::*;
    pub use super::input::*;
    pub use super::movement::*;
    pub use super::ai::*;
}

use macroquad::prelude::*;
use shipyard::*;
use crate::game::prelude::*;