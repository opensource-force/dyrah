use super::*;
use collections::storage;
use systems::prelude::*;
use map::{Map, TILE_OFFSET, TILE_SIZE};

pub struct Game {
    world: World,
    map: Map
}

impl Game {
    pub async fn new() -> Self {
        let mut world = World::new();
        storage::store(WorldTime(get_time()));
        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        let _player_id = world.add_unique(Player {
            pos: Position(Vec2::ZERO),
            vel: Velocity(Vec2::ZERO),
            spr: Sprite {
                tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                frame: ivec2(1, 4)
            },
            moving: Moving(false),
            target_pos: TargetPosition(Vec2::ZERO),
            target: Target(None)
        });
        let _monster_ids = world.bulk_add_entity((0..199).map(|_| (
            Monster,
            Position(vec2(
                rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                rand::gen_range(0.0, 64.0 * TILE_SIZE.y)
            )),
            Velocity(Vec2::ZERO),
            Sprite {
                tex: monster_tex.clone(),
                frame: ivec2(rand::gen_range(0, 1), rand::gen_range(0, 7))
            }
        )));

        world.add_unique(Camera(Camera2D::from_display_rect(Rect::new(
            0.0, 0.0, screen_width(), -screen_height()
        ))));

        Self {
            world,
            map: Map::new("assets/map.json", "assets/tiles.png").await
        }
    }

    pub fn events(&self) {
        self.world.run(InputSystem::control_player);
        self.world.run(AiSystem::control_monsters);
    }

    pub fn update(&mut self) {
        if let Ok(mut player) = self.world.get_unique::<&mut Player>() {
            if let Some(tile) = self.map.get_tile(player.target_pos.0) {
                if tile.walkable {
                    player.moving.0 = true;
                    player.target_pos.0 = tile.rect.center();
                }
            } else {
                player.moving.0 = false;
            }

            let mut camera = self.world.get_unique::<&mut Camera>().unwrap();
            camera.0.target = player.pos.0;

            self.map.update(Rect::new(
                player.pos.0.x - screen_width() / 2.0 - TILE_SIZE.x,
                player.pos.0.y - screen_height() / 2.0 - TILE_SIZE.y,
                screen_width() + TILE_SIZE.x,
                screen_height() + TILE_SIZE.y
            ));
        }

        self.world.run(MovementSystem::update);
    }

    pub fn draw(&mut self) {
        clear_background(SKYBLUE);

        self.map.draw();
        self.world.run(RenderSystem::draw_entities);

        if let Ok(camera) = self.world.get_unique::<&Camera>() {
            set_camera(&camera.0);
        }

        self.world.run(RenderSystem::debug);

        if let Ok(player) = self.world.get_unique::<&Player>() {
            if let Some(target) = player.target.0 {
                let monster_pos = self.world.get::<&Position>(target).unwrap();
                
                draw_rectangle_lines(
                    monster_pos.0.x - TILE_OFFSET.x, monster_pos.0.y - TILE_OFFSET.y,
                    TILE_SIZE.x, TILE_SIZE.y,
                    2.0, PURPLE
                )
            }
        }
    }
}