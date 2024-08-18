use macroquad::{camera::{set_camera, Camera2D}, math::{Rect, Vec2}};

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

    pub fn update(&mut self, pos: Vec2, width: f32, height: f32) {
        if width != self.width || height != self.height {
            self.camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, -height));
            (self.width, self.height) = (width, height);
        }
        
        self.camera.target = pos;
    }

    pub fn draw(&self) {
        set_camera(&self.camera);
    }
}