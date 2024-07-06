mod systems;
mod game;
mod map;

pub mod prelude {
    pub use super::game::*;
    pub use super::map::*;
}

use super::*;
use collections::storage;

// collections
pub struct WorldTime(f64);
pub struct PlayerView(Rect);


// components
pub struct Player;
pub struct Monster;

pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);
pub struct Sprite {
    pub texture: Texture2D,
    pub frame: IVec2
}
pub struct Moving(pub bool);
pub struct TargetPosition(pub Vec2);