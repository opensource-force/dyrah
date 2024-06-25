use super::*;

pub trait Quad {
    fn draw(&self, color: Color, offset: Option<Vec2>);
    fn intersects(&self, rect: &Rect, offset: Option<Vec2>) -> bool;
}

impl Quad for Rect {
    fn draw(&self, color: Color, offset: Option<Vec2>) {
        draw_rectangle_lines(
            self.x + offset.unwrap_or_default().x,
            self.y + offset.unwrap_or_default().y,
            self.w, self.h,
            2.0, color
        );
    }

    fn intersects(&self, rect: &Rect, offset: Option<Vec2>) -> bool {
        let this = vec2(self.x, self.y) + offset.unwrap_or_default();

        rect.x + rect.w >= this.x
            && rect.x <= this.x + rect.w
            && rect.y + rect.h >= this.y
            && rect.y <= this.y + rect.h
    }
}