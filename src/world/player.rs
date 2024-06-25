use super::*;

pub trait Player {
    fn keyboard_controller(&mut self);
}

impl Player for Entity {
    fn keyboard_controller(&mut self) {
        (self.velocity, self.animation) = if
            is_key_down(KeyCode::W) || is_key_down(KeyCode::Up)
        {
            (vec2(1.0, -0.5), 4)
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            (vec2(-1.0, -0.5), 5)
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            (vec2(-1.0, 0.5), 6)
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            (vec2(1.0, 0.5), 7)
        } else {
            (Vec2::ZERO, match self.sprite.current_animation() {
                4 => 0, 5 => 1, 6 => 2, 7 => 3, _ => return
            })
        };
    }
}