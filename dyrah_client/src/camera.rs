use macroquad::{
    camera::{Camera2D, set_camera},
    math::{Rect, vec2},
    window::{screen_height, screen_width},
};

pub struct Camera {
    pub inner: Camera2D,
    width: f32,
    height: f32,
}

impl Camera {
    pub fn attach_sized(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if width != self.width || height != self.height {
            let size = Rect::new(0., 0., width, -height);
            self.inner = Camera2D::from_display_rect(size);

            (self.width, self.height) = (width, height);
        }

        self.inner.target = vec2(x, y);
    }

    pub fn set(&self) {
        set_camera(&self.inner);
    }
}

impl Default for Camera {
    fn default() -> Self {
        let (w, h) = (screen_width(), screen_height());
        let size = Rect::new(0., 0., w, -h);
        let mut camera = Camera2D::from_display_rect(size);

        camera.target = vec2(0., 0.);
        set_camera(&camera);

        Self {
            inner: camera,
            width: w,
            height: h,
        }
    }
}
