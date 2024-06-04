use super::*;

use map::*;
use entity::*;

pub struct World {
    map: Map,
    camera: Camera2D,
    player: Entity
}

impl World {
    pub async fn new(tex: Texture2D) -> Self {
        let camera = Camera2D::from_display_rect(Rect::new(0.0, screen_width(), screen_width(), -screen_height()));

        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            camera,
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

        self.camera.target = Vec2::new(
            self.player.rect.x + self.player.rect.w / 2.0,
            self.player.rect.y + self.player.rect.h / 2.0
        );
    }

    pub fn draw(&self) {
        set_camera(&self.camera);

        self.map.draw();
        self.player.draw();
    }
}