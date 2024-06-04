use super::*;

use entity::*;

pub struct World {
    player: Entity
}

impl World {
    pub fn new(tex: Texture2D) -> Self {
        clear_background(SKYBLUE);

        Self {
            player: Entity::new(
                screen_width()/2.0,
                screen_height()/2.0,
                32.0, 32.0,
                tex
            )
        }
    }

    pub fn update(&mut self) {
        self.player.update();
    }

    pub fn draw(&self) {
        self.player.draw();
    }
}