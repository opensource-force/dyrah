use super::*;
use map::*;
use entity::*;

use animation::Animation;

pub const TILE_SIZE: Vec2 = vec2(32.0, 32.0);
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
        let wolf_animations = &[
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

        for _ in 0..50 {
            enemies.push(Entity::new(
                Rect::new(
                    rand::gen_range(-640.0, 640.0),
                    rand::gen_range(320.0, 1280.0),
                    TILE_SIZE.x, TILE_SIZE.y
                ),
                1.0,
                WOLF_TEX_PATH, wolf_animations
            ).await)
        }

        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            camera: Camera2D::from_display_rect(Rect::new(
                0.0, 0.0,
                screen_width(), -screen_height()
            )),
            player: Entity::new(
                Rect::new(0.0, 640.0, TILE_SIZE.x, TILE_SIZE.y),
                4.0,
                WOLF_TEX_PATH, wolf_animations
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

        self.map.update(self.player.rect.x, self.player.rect.y);
        self.player.update();
        self.player.keyboard_controller();

        for tile in &self.map.chunk {
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
            
            for tile in &self.map.chunk {
                if !tile.walkable {
                    if enemy.aabb(&tile.rect) {
                        println!("{}: Enemy encounted a prop", get_time());

                        enemy.collide(&tile.rect);
                        break;
                    }
                }
            }
        }

        if get_time() - self.time > 2.0 {
            for enemy in &mut self.enemies {
                enemy.ai_controller();
            }

            self.time = get_time();
        }
    }

    pub fn draw(&mut self) {
        self.map.draw();

        for tile in &self.map.chunk {
            if !tile.walkable {
                draw_rectangle_lines(
                    tile.rect.x + tile.rect.w / 2.0,
                    tile.rect.y + tile.rect.h / 2.0,
                    tile.rect.w, tile.rect.h,
                    2.0, GREEN
                );
            }
        }

        self.player.draw();
        
        draw_rectangle_lines(
            self.player.rect.x + self.player.rect.w / 2.0,
            self.player.rect.y + self.player.rect.h / 2.0,
            self.player.rect.w, self.player.rect.h,
            2.0, BLUE
        );

        set_camera(&self.camera);
        
        for enemy in &mut self.enemies {
            if self.player.rect.x < enemy.rect.x + screen_width() / 2.0
                && self.player.rect.x > enemy.rect.x - screen_width() / 2.0 + enemy.rect.w
                && self.player.rect.y < enemy.rect.y + screen_height() / 2.0
                && self.player.rect.y > enemy.rect.y - screen_height() / 2.0 + enemy.rect.h
            {
                enemy.draw();

                draw_rectangle_lines(
                    enemy.rect.x + enemy.rect.w / 2.0,
                    enemy.rect.y + enemy.rect.h / 2.0,
                    enemy.rect.w, enemy.rect.h,
                    2.0, RED
                );
            }
        }
    }
}

pub fn world_to_map(world_pos: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE;
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE;
    let inverse = mat2(ihat, jhat).inverse();

    inverse.mul_vec2(world_pos)
}

pub fn map_to_world(map_pos: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE;
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE;
    let transform = mat2(ihat, jhat);
    let offset = vec2(-TILE_SIZE.x / 2.0, 0.0);

    transform.mul_vec2(map_pos) + offset
}