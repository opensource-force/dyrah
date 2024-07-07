mod render;
mod input;
mod movement;

pub mod prelude {
    pub use super::render::*;
    pub use super::input::*;
    pub use super::movement::*;
}

use super::*;
use map::*;