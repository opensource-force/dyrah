use super::*;
use map::*;
use entity::*;

pub struct World {
    map: Map,
    camera: Camera2D,
    player: Entity,
    enemies: Vec<Entity>,
    time: f64
}

impl World {
    pub async fn new() -> Self {
        let wolf_animations = vec![
            Entity::animation("idle_up", 11, 8, 5),
            Entity::animation("idle_left", 10, 8, 5),
            Entity::animation("idle_down", 8, 8, 5),
            Entity::animation("idle_right", 9, 8, 5),
            Entity::animation("walk_up", 15, 8, 15),
            Entity::animation("walk_left", 14, 8, 15),
            Entity::animation("walk_down", 12, 8, 15),
            Entity::animation("walk_right", 13, 8, 15)
        ];
        let mut enemies = Vec::new();
        
        for _ in 0..25 {
            enemies.push(
                Entity::new(
                    rand::gen_range(-640.0, 640.0),
                    rand::gen_range(320.0, 1280.0),
                    32.0, 32.0,
                    "assets/critters/wolf/wolf-all.png",
                    wolf_animations.clone()
                ).await
            )
        }

        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            camera: Camera2D::from_display_rect(Rect::new(
                0.0, 0.0,
                screen_width(), -screen_height()
            )),
            player: Entity::new(
                -800.0, 1600.0, 32.0, 32.0,
                "assets/critters/wolf/wolf-all.png",
                wolf_animations
            ).await,
            enemies,
            time: get_time()
        }
    }

    pub fn update(&mut self) {
        self.camera.target = vec2(
            self.player.rect.x + self.player.rect.w / 2.0,
            self.player.rect.y + self.player.rect.h / 2.0
        );

        //self.map.update();
        self.player.update();
        self.player.keyboard_controller();

        for collider in &self.map.colliders {
            if self.player.aabb(collider) {
                println!("{}: Encountered a prop", get_time());
            }
        }

        for enemy in &mut self.enemies {
            enemy.update();
            
            if self.player.aabb(&enemy.rect) {
                println!("{}: Encountered an enemy", get_time());
            }

            /* not the most performant
            for collider in &self.map.colliders {
                if enemy.aabb(collider) {
                    println!("{}: Enemy encounted a prop", get_time());
                }
            }
            */
        }

        if get_time() - self.time > 1.0 {
            for enemy in &mut self.enemies {
                enemy.ai_controller();
            }

            self.time = get_time();
        }
    }

    pub fn draw(&mut self) {
        set_camera(&self.camera);

        self.map.draw();
        self.player.draw(4.0);
        
        for enemy in &mut self.enemies {
            enemy.draw(1.0);
        }
    }
}