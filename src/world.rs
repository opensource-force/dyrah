use super::*;

use map::*;
use entity::*;

pub struct World {
    map: Map,
    camera: Camera2D,
    player: Entity
}

impl World {
    pub async fn new() -> Self {
        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            camera: Camera2D::from_display_rect(Rect::new(
                0.0, 0.0,
                screen_width(), -screen_height()
            )),
            player: Entity::new(
                0.0, 0.0,
                32.0, 32.0,
                "assets/critters/wolf/wolf-all.png",
                vec![
                    Entity::anamation("idle_up", 11, 8, 5),
                    Entity::anamation("idle_left", 10, 8, 5),
                    Entity::anamation("idle_down", 8, 8, 5),
                    Entity::anamation("idle_right", 9, 8, 5),
                    Entity::anamation("walk_up", 15, 8, 15),
                    Entity::anamation("walk_left", 14, 8, 15),
                    Entity::anamation("walk_down", 12, 8, 15),
                    Entity::anamation("walk_right", 13, 8, 15)
                ]
            ).await
        }
    }

    pub fn update(&mut self) {
        self.camera.target = Vec2::new(
            self.player.rect.x + self.player.rect.w / 2.0,
            self.player.rect.y + self.player.rect.h / 2.0
        );
        
        //self.map.update();
        self.player.update();
        self.player.handle_input();
    }

    pub fn draw(&mut self) {
        set_camera(&self.camera);

        self.map.draw();
        self.player.draw();
    }
}