mod quad;
mod map;
mod entity;
pub mod isometric;

pub mod prelude {
    pub use crate::engine::quad::*;
    pub use crate::engine::map::*;
    pub use crate::engine::entity::*;
}

use macroquad::prelude::*;