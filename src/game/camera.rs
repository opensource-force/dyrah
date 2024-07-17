use super::*;

#[derive(Unique)]
pub struct Viewport(pub Camera2D);

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self(Camera2D::from_display_rect(Rect::new(
            0.0, 0.0, width, -height
        )))
    }

    pub fn update(&mut self, position: Vec2) {
        self.0.target = position;
    }

    pub fn draw(&self) {
        set_camera(&self.0);
    }
}