use super::*;

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D
}

impl Entity {
    pub fn new(x: f32, y: f32, w: f32, h: f32, tex: Texture2D) -> Self {
        Self {
            rect: Rect::new(x, y, w, h),
            tex
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.rect.y -= 16.0;
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.rect.x -= 16.0;
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.rect.y += 16.0;
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.rect.x += 16.0;
        }
    }
    
    pub fn draw(&self) {
        let rect = self.rect;
        let tex = &self.tex;

        draw_texture_ex(tex, rect.x, rect.y, WHITE, DrawTextureParams {
            dest_size: Some(Vec2::new(rect.w, rect.h)),
            ..Default::default()
        });
    }    
}