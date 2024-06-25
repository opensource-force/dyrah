use super::*;
use collections::storage;
use animation::Animation;
use macroquad::ui::root_ui;

pub const TILE_SIZE: Vec2 = vec2(32.0, 32.0);
pub const TILE_OFFSET: Vec2 = vec2(TILE_SIZE.x / 2.0, TILE_SIZE.y / 2.0);
const WOLF_TEX_PATH: &str = "assets/critters/wolf/wolf-all.png";

pub struct WorldTime(f64);
pub struct PlayerView(Rect);

pub struct World {
    map: Map,
    camera: Camera2D,
    player: Entity,
    enemies: Vec<Entity>
}

impl World {
    pub async fn new() -> Self {
        storage::store(WorldTime(get_time()));

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
            map: Map::new(
                "assets/tilemap.json", "assets/tileset.png",
                true
            ).await,
            camera: Camera2D::from_display_rect(Rect::new(
                0.0, 0.0, screen_width(), -screen_height()
            )),
            player: Entity::new(
                Rect::new(0.0, 640.0, TILE_SIZE.x, TILE_SIZE.y),
                4.0,
                WOLF_TEX_PATH, wolf_animations
            ).await,
            enemies
        }
    }

    pub fn update(&mut self) {
        clear_background(SKYBLUE);

        self.player.keyboard_controller();
        self.player.update();

        if self.player.moving {
            self.player.moving = false;
            self.camera.target = self.player.rect.center();

            storage::store(PlayerView(Rect::new(
                self.player.rect.x - screen_width() / 2.0 - TILE_SIZE.x,
                self.player.rect.y - screen_height() / 2.0 - TILE_SIZE.y,
                screen_width() + TILE_SIZE.x,
                screen_height() + TILE_SIZE.y
            )));

            self.map.update(
                &["base", "colliders"],
                storage::get::<PlayerView>().0,
                TILE_SIZE
            );

            for tile in &self.map.chunk {
                if !tile.walkable {
                    if tile.rect.intersects(&self.player.rect, None) {
                        //self.player.collide();
                        break;
                    }
                }
            }
        }

        if get_time() - storage::get::<WorldTime>().0 > 2.0 {
            for enemy in &mut self.enemies {
                enemy.ai_controller();
            }

            storage::store(WorldTime(get_time()));
        }

        for enemy in &mut self.enemies {
            enemy.update();

            if enemy.moving {
                for tile in &self.map.chunk {
                    if !tile.walkable {
                        if tile.rect.intersects(&enemy.rect, None) {
                            enemy.collide();
                            break;
                        }
                    }
                }
            }

            if enemy.moving || self.player.moving {
                if enemy.rect.intersects(&self.player.rect, None) {
                    self.player.collide();
                    enemy.collide();
                    break;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        root_ui().label(None, &format!("FPS: {}", get_fps()));

        self.map.draw();

        for tile in &self.map.chunk {
            if !tile.walkable {
                tile.rect.draw(GREEN, Some(TILE_OFFSET));
            }
        }

        self.player.draw();
        self.player.rect.draw(BLUE, Some(TILE_OFFSET));

        set_camera(&self.camera);
        
        for enemy in &mut self.enemies {
            if storage::get::<PlayerView>().0.contains(enemy.rect.point()) {
                enemy.draw();
                enemy.rect.draw(RED, Some(TILE_OFFSET));
            }
        }
    }
}