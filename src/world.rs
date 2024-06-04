use super::*;

use map::*;
use entity::*;

pub struct World {
    map: Map,
    player: Entity
}

impl World {
    pub async fn new(tex: Texture2D) -> Self {
        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            player: Entity::new(
                screen_width()/2.0,
                screen_height()/2.0,
                32.0, 32.0,
                tex
            )
        }
    }

    pub fn update(&mut self) {
        self.map.update();
        self.player.update();
    }

    pub fn draw(&self) {
        self.map.draw();
        self.player.draw();
    }
}