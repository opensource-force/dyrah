use super::*;
use animation::Animation;
use map::*;
use entity::*;

const WOLF_TEX_PATH: &str = "assets/critters/wolf/wolf-all.png";

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
            Animation { name: "idle_up".to_string(), row: 11, frames: 8, fps: 5 },
            Animation { name: "idle_left".to_string(), row: 10, frames: 8, fps: 5 },
            Animation { name: "idle_down".to_string(), row: 8, frames: 8, fps: 5 },
            Animation { name: "idle_right".to_string(), row: 9, frames: 8, fps: 5 },
            Animation { name: "walk_up".to_string(), row: 15, frames: 8, fps: 15 },
            Animation { name: "walk_left".to_string(), row: 14, frames: 8, fps: 15 },
            Animation { name: "walk_down".to_string(), row: 12, frames: 8, fps: 15 },
            Animation { name: "walk_right".to_string(), row: 13, frames: 8, fps: 15 }
        ];
        let mut enemies = Vec::new();
        
        for _ in 0..100 {
            enemies.push(
                Entity::new(
                    rand::gen_range(-640.0, 640.0),
                    rand::gen_range(320.0, 1280.0),
                    32.0, 32.0,
                    WOLF_TEX_PATH,
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
                0.0, 640.0, 32.0, 32.0,
                WOLF_TEX_PATH,
                wolf_animations
            ).await,
            enemies,
            time: get_time()
        }
    }

    pub fn update(&mut self) {
        draw_text(&format!("FPS: {}", get_fps()), -25.0, 0.0, 30.0, BLUE);

        self.camera.target = vec2(
            self.player.rect.x + self.player.rect.w / 2.0,
            self.player.rect.y + self.player.rect.h / 2.0
        );

        self.map.update();
        self.player.update();
        self.player.keyboard_controller();

        for tile in &self.map.tiles {
            if !tile.walkable {
                if self.player.aabb(&tile.rect) {
                    println!("{}: Encountered a prop", get_time());

                    self.player.collide(&tile.rect);
                    break;
                }
            }
        }

        for enemy in &mut self.enemies {
            enemy.update();

            if self.player.aabb(&enemy.rect) {
                println!("{}: Encountered an enemy", get_time());

                self.player.collide(&enemy.rect);
                break;
            }
            
            for tile in &self.map.tiles {
                if !tile.walkable {
                    if enemy.aabb(&tile.rect) {
                        println!("{}: Enemy encounted a prop", get_time());

                        enemy.collide(&tile.rect);
                        break;
                    }
                }
            }
        }

        if get_time() - self.time > 1.0 {
            for enemy in &mut self.enemies {
                enemy.ai_controller();
            }

            self.time = get_time();
        }
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.player.draw(4.0);

        set_camera(&self.camera);
        
        for enemy in &mut self.enemies {
            enemy.draw(1.0);
        }
    }
}