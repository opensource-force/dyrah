mod render;
mod input;
mod ai;
mod movement;
mod damage;
mod removal;

pub mod prelude {
    pub use super::render::*;
    pub use super::input::*;
    pub use super::ai::*;
    pub use super::movement::*;
    pub use super::damage::*;
    pub use super::removal::*;
}

use macroquad::prelude::*;
use shipyard::*;
use crate::game::prelude::*;
