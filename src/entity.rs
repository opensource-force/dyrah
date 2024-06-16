use super::*;

use animation::{AnimatedSprite, Animation};

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    pub velocity: Vec2,
    pub speed: f32,
    sprite: AnimatedSprite,
    animation: usize,
}

impl Entity {
    pub async fn new(
        rect: Rect, speed: f32,
        tex_path: &str, animations: &[Animation]
    ) -> Self {
        Self {
            rect,
            tex: load_texture(tex_path).await.unwrap(),
            sprite: AnimatedSprite::new(
                64, 64,
                animations, true
            ),
            speed,
            animation: 0,
            velocity: Vec2::ZERO,
        }
    }

    pub fn update(&mut self) {
        self.rect.x += self.velocity.x * self.speed;
        self.rect.y += self.velocity.y * self.speed;

        self.sprite.update();
    }
    
    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.tex,
            self.rect.x, self.rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                ..Default::default()
            }
        );

        self.sprite.set_animation(self.animation);
    }

    pub fn ai_controller(&mut self) {
        (self.velocity, self.animation) = match rand::gen_range(0, 21) {
            0 => (vec2(1.0, -0.5), 4),
            1 => (vec2(-1.0, -0.5), 5),
            2 => (vec2(-1.0, 0.5), 6),
            3 => (vec2(1.0, 0.5), 7),
            _ => {
                (Vec2::ZERO, match self.sprite.current_animation() {
                    4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
                })
            }
        };
    }

    pub fn keyboard_controller(&mut self) {
        (self.velocity, self.animation) = if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            (vec2(1.0, -0.5), 4)
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            (vec2(-1.0, -0.5), 5)
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            (vec2(-1.0, 0.5), 6)
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            (vec2(1.0, 0.5), 7)
        } else {
            (Vec2::ZERO, match self.sprite.current_animation() {
                4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
            })
        };
    }

    pub fn aabb(&mut self, rect: &Rect) -> bool {
        if rect.x + rect.w >= self.rect.x
            && rect.x <= self.rect.x + rect.w
            && rect.y + rect.h >= self.rect.y
            && rect.y <= self.rect.y + rect.h
        {
            return true
        }

        return false
    }

    pub fn collide(&mut self, rect: &Rect) {
        let push = (self.rect.center() - rect.center()).normalize();
            
        self.velocity = Vec2::ZERO;
        self.rect.x += push.x;
        self.rect.y += push.y;
    }
}