use super::*;

#[derive(Unique)]
pub struct Viewport {
    pub camera: Camera2D,
    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, -height)),
            width,
            height,
        }
    }

    pub fn update(&mut self, position: Vec2, width: f32, height: f32) {
        if width != self.width || height != self.height {
            self.camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, -height));
            self.width = width;
            self.height = height;
        }
        self.camera.target = position;
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
    }
}
