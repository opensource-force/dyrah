use super::*;

use animation::{AnimatedSprite, Animation};

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    sprite: AnimatedSprite,
    pub velocity: Vec2
}

impl Entity {
    pub async fn new(
        x: f32, y: f32, w: f32, h: f32,
        tex_path: &str, animations: Vec<Animation>
    ) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            tex: load_texture(tex_path).await.unwrap(),
            sprite: AnimatedSprite::new(
                64, 64,
                &animations, true
            ),
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
            self.rect.x, self.rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(self.sprite.frame().source_rect),
                dest_size: Some(self.sprite.frame().dest_size),
                ..Default::default()
            }
        );
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

    pub fn aabb(&mut self, rect: &Rect) -> bool {
        if rect.x + rect.w >= self.rect.x
            && rect.x <= self.rect.x + rect.w
            && rect.y + rect.w >= self.rect.y
            && rect.y <= self.rect.y + rect.w
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