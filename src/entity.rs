use animation::{AnimatedSprite, Animation};

use super::*;

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    sprite: AnimatedSprite
}

impl Entity {
    pub async fn new(x: f32, y: f32, w: f32, h: f32, tex_path: &str, animations: Vec<Animation>) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            tex: load_texture(tex_path).await.unwrap(),
            sprite: AnimatedSprite::new(64, 64, &animations, true)
        }
    }

    pub fn update(&mut self) {
        self.sprite.update();
    }
    
    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.tex,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                ..Default::default()
            }
        );
    }
}

pub trait Animator {
    fn anamation(name: &str, row: u32, cols: u32, fps: u32) -> Animation;
    fn animate(&mut self, index: usize);
}

impl Animator for Entity {
    fn anamation(name: &str, row: u32, cols: u32, fps: u32) -> Animation {
        Animation { name: name.to_string(), row, frames: cols, fps }
    }
    fn animate(&mut self, index: usize) {
        self.sprite.set_animation(index);
    }
}

pub trait InputHandler {
    fn handle_input(&mut self);
}

impl InputHandler for Entity {
    fn handle_input(&mut self) {
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.rect.x += 8.0;
            self.rect.y -= 8.0;
            self.animate(4);
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.rect.x -= 8.0;
            self.rect.y -= 8.0;
            self.animate(5);
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.rect.x -= 8.0;
            self.rect.y += 8.0;
            self.animate(6);
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.rect.x += 8.0;
            self.rect.y += 8.0;
            self.animate(7);
        } else {
            match self.sprite.current_animation() {
                4 => self.animate(0),
                5 => self.animate(1),
                6 => self.animate(2),
                7 => self.animate(3),
                _ => {}
            }
        }
    }
}