use super::*;

pub trait Enemy {
    fn ai_controller(&mut self);
}

impl Enemy for Entity {
    fn ai_controller(&mut self) {
        (self.velocity, self.animation) = match rand::gen_range(0, 21) {
            0 => (vec2(1.0, -0.5), 4),
            1 => (vec2(-1.0, -0.5), 5),
            2 => (vec2(-1.0, 0.5), 6),
            3 => (vec2(1.0, 0.5), 7),
            _ => {
                (Vec2::ZERO, match self.sprite.current_animation() {
                    4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
                })
            }
        };
    }
}