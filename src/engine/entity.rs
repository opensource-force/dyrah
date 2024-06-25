use super::*;
use animation::{AnimatedSprite, Animation};

pub struct Entity {
    pub rect: Rect,
    tex: Texture2D,
    pub sprite: AnimatedSprite,
    pub animation: usize,
    pub velocity: Vec2,
    pub speed: f32,
    pub moving: bool,
    pub last_pos: Vec2,
    pub target: Vec2
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
            animation: 0,
            velocity: Vec2::ZERO,
            speed,
            moving: false,
            last_pos: Vec2::ZERO,
            target: vec2(rect.x, rect.y)
        }
    }

    pub fn update(&mut self) {
        self.moving = true;
        self.last_pos = self.rect.point();
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

    pub fn collide(&mut self) {
        self.moving = false;
        self.velocity = Vec2::ZERO;
        self.rect.x = self.last_pos.x;
        self.rect.y = self.last_pos.y;
    }
}