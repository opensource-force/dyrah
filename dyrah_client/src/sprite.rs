use egor::math::Vec2;

#[derive(Debug)]
struct Frame {
    uv_coords: [[f32; 2]; 4],
    duration: f32,
}

#[derive(Debug)]
pub struct Animation {
    frames: Vec<Frame>,
    timer: f32,
    current: usize,
    flipped_x: bool,
    flipped_y: bool,
}

impl Animation {
    pub fn new(rows: usize, cols: usize, total: usize, dur: f32) -> Self {
        let mut frames = Vec::with_capacity(total);
        let (fw, fh) = (1.0 / cols as f32, 1.0 / rows as f32);
        for i in 0..total {
            let (x, y) = ((i % cols) as f32 * fw, (i / cols) as f32 * fh);
            frames.push(Frame {
                uv_coords: [[x, y], [x + fw, y], [x + fw, y + fh], [x, y + fh]],
                duration: dur,
            });
        }
        Self {
            frames,
            timer: 0.0,
            current: 0,
            flipped_x: false,
            flipped_y: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.frames.is_empty() {
            return;
        }

        self.timer += dt;
        if self.timer >= self.frames[self.current].duration {
            self.timer = 0.0;
            self.current = (self.current + 1) % self.frames.len();
        }
    }

    pub fn frame(&self) -> [[f32; 2]; 4] {
        let mut uv = self.frames[self.current].uv_coords;

        if self.flipped_x {
            uv.swap(0, 1); // TL <-> TR
            uv.swap(3, 2); // BL <-> BR
        }
        if self.flipped_y {
            uv.swap(0, 3); // TL <-> BL
            uv.swap(1, 2); // TR <-> BR
        }

        uv
    }
    pub fn set_frame(&self, f: usize) -> [[f32; 2]; 4] {
        self.frames[f].uv_coords
    }

    pub fn flip_x(&mut self, flip: bool) {
        self.flipped_x = flip;
    }
    pub fn flip_y(&mut self, flip: bool) {
        self.flipped_y = flip;
    }

    pub fn offset(&self, frame_size: Vec2, sprite_size: Vec2) -> Vec2 {
        let mut offset = Vec2::ZERO;

        if self.flipped_x {
            offset.x = -(frame_size.x - sprite_size.x);
        }

        if self.flipped_y {
            offset.y = -(frame_size.y - sprite_size.y);
        }

        offset
    }
}
