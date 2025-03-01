use macroquad::texture::Texture2D;

pub mod game;
mod map;
mod camera;

pub struct PlayerSprite {
    texture: Texture2D,
    frame: (f32, f32)
}