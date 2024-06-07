use animation::{AnimatedSprite, Animation};

use super::*;

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    sprite: AnimatedSprite,
    velocity: Vec2
}

impl Entity {
    pub async fn new(x: f32, y: f32, w: f32, h: f32, tex_path: &str, animations: Vec<Animation>) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            tex: load_texture(tex_path).await.unwrap(),
            sprite: AnimatedSprite::new(64, 64, &animations, true),
            velocity: vec2(0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        self.sprite.update();
    }
    
    pub fn draw(&mut self, speed: f32) {
        self.rect.x += self.velocity.x * speed;
        self.rect.y += self.velocity.y * speed;

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

    pub fn animation(name: &str, row: u32, cols: u32, fps: u32) -> Animation {
        Animation { name: name.to_string(), row, frames: cols, fps }
    }

    pub fn ai_controller(&mut self) {
        let (velocity, animation) = match rand::gen_range(0, 7) {
            0 => (vec2(1.0, -1.0), 4),
            1 => (vec2(-1.0, -1.0), 5),
            2 => (vec2(-1.0, 1.0), 6),
            3 => (vec2(1.0, 1.0), 7),
            _ => {
                (Vec2::ZERO, match self.sprite.current_animation() {
                    4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
                })
            }
        };

        self.velocity = velocity;
        self.sprite.set_animation(animation);
    }

    pub fn keyboard_controller(&mut self) {
        let (velocity, animation) = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            (vec2(1.0, -1.0), 4)
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            (vec2(-1.0, -1.0), 5)
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            (vec2(-1.0, 1.0), 6)
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            (vec2(1.0, 1.0), 7)
        } else {
            (Vec2::ZERO, match self.sprite.current_animation() {
                4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
            })
        };

        self.velocity = velocity;
        self.sprite.set_animation(animation);
    }
}