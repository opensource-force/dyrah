use dyrah_shared::TILE_SIZE;
use macroquad::{
    math::Vec2,
    prelude::animation::{AnimatedSprite, Animation},
    texture::Texture2D,
};

mod camera;
pub mod game;
mod map;
mod systems;

pub const SPRITE_SIZE: f32 = 64.;

struct SpriteFlip {
    x: bool,
    y: bool,
}

pub struct Sprite {
    animation: AnimatedSprite,
    frame: (f32, f32),
    is_flipped: SpriteFlip,
}

impl Sprite {
    fn new(animations: &[Animation]) -> Self {
        Self {
            animation: AnimatedSprite::new(
                SPRITE_SIZE as u32,
                SPRITE_SIZE as u32,
                animations,
                true,
            ),
            frame: (0.0, 0.0),
            is_flipped: SpriteFlip { x: false, y: false },
        }
    }

    fn from_frame(x: f32, y: f32) -> Self {
        Self {
            animation: AnimatedSprite::new(0, 0, &[], false),
            frame: (x * TILE_SIZE, y * TILE_SIZE),
            is_flipped: SpriteFlip { x: false, y: false },
        }
    }
}

pub struct Player {
    is_attacking: bool,
}

pub struct Creature;
pub struct PlayerTexture(Texture2D);
pub struct CreatureTexture(Texture2D);

pub struct Damage {
    pub origin: u64,
    pub value: f32,
    pub position: Vec2,
    pub lifetime: f32,
}

#[derive(Default)]
pub struct Damages {
    pub numbers: Vec<Damage>,
}
