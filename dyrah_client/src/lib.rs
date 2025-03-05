use macroquad::texture::Texture2D;

mod camera;
pub mod game;
mod map;

pub struct PlayerSprite {
    texture: Texture2D,
    frame: (f32, f32),
}
