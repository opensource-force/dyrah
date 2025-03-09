use macroquad::texture::Texture2D;

mod camera;
pub mod game;
mod map;

pub struct PlayerTexture(Texture2D);
pub struct CreatureTexture(Texture2D);

pub struct PlayerSprite {
    frame: (f32, f32),
}

pub struct CreatureSprite {
    frame: (f32, f32),
}
