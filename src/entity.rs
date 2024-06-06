use animation::{AnimatedSprite, Animation};

use super::*;

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    sprite: AnimatedSprite
}

impl Entity {
    pub async fn new(x: f32, y: f32, w: f32, h: f32, path: &str) -> Self {
        let tex = load_texture(path).await.unwrap();
        let mut sprite = AnimatedSprite::new(
            64, 64, &[
                Animation {
                    name: "idle_up".to_string(),
                    row: 11,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "idle_left".to_string(),
                    row: 10,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "idle_down".to_string(),
                    row: 8,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "idle_right".to_string(),
                    row: 9,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "walk_up".to_string(),
                    row: 15,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "walk_left".to_string(),
                    row: 14,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "walk_down".to_string(),
                    row: 12,
                    frames: 8,
                    fps: 8
                },
                Animation {
                    name: "walk_right".to_string(),
                    row: 13,
                    frames: 8,
                    fps: 8
                }
            ], true
        );

        sprite.set_animation(0);

        Self {
            rect: Rect::new(x, y, w, h),
            tex,
            sprite
        }
    }

    pub fn update(&mut self) {
        if is_key_released(KeyCode::W) || is_key_released(KeyCode::Up) {
            self.sprite.set_animation(0);
        } else if is_key_released(KeyCode::S) || is_key_released(KeyCode::Left) {
            self.sprite.set_animation(1);
        } else if is_key_pressed(KeyCode::A) || is_key_released(KeyCode::Down) {
            self.sprite.set_animation(2);
        } else if is_key_released(KeyCode::D) || is_key_released(KeyCode::Right) {
            self.sprite.set_animation(3);
        } else if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.rect.x += 8.0;
            self.rect.y -= 8.0;
            self.sprite.set_animation(4);
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.rect.x -= 8.0;
            self.rect.y -= 8.0;
            self.sprite.set_animation(5);
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.rect.x -= 8.0;
            self.rect.y += 8.0;
            self.sprite.set_animation(6);
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.rect.x += 8.0;
            self.rect.y += 8.0;
            self.sprite.set_animation(7);
        }
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

        self.sprite.update();
    }    
}