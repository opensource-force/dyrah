use egor::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::sprite::Animation;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WorldPos {
    pub vec: Vec2,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TargetWorldPos {
    pub vec: Vec2,
}

#[derive(Debug)]
pub struct Sprite {
    pub anim: Animation,
    pub frame_size: Vec2,
    pub sprite_size: Vec2,
}
