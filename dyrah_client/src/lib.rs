use dyrah_shared::map::TILE_SIZE;
use macroquad::texture::Texture2D;

mod camera;
pub mod game;
mod map;

pub struct Sprite {
    frame: (f32, f32),
}

pub struct Player {
    sprite: Sprite,
}

pub struct Creature;
pub struct MapTexture(Texture2D);
pub struct PlayerTexture(Texture2D);
pub struct CreatureTexture(Texture2D);

impl Sprite {
    fn from_frame(x: f32, y: f32) -> Self {
        Self {
            frame: (x * TILE_SIZE, y * TILE_SIZE),
        }
    }
}
