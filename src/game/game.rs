use super::*;
use map::{Map, TILE_OFFSET, TILE_SIZE};
use systems::prelude::*;

pub struct Game {
    world: World,
    map: Map,
    camera: Camera2D
}

impl Game {
    pub async fn new() -> Self {
        storage::store(WorldTime(get_time()));

        let mut world = World::new();
        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        world.spawn((
            Player,
            Position(TILE_OFFSET),
            Velocity(Vec2::ZERO),
            Sprite {
                texture: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                frame: ivec2(1, 4)
            },
            Moving(false),
            TargetPosition(vec2(TILE_OFFSET.x, TILE_OFFSET.y)),
            Health(100.0),
            Target(None)
        ));
        
        let monsters = (0..99).map(|_| {(
            Monster,
            Position(vec2(
                rand::gen_range(16.0, 64.0 * TILE_SIZE.x),
                rand::gen_range(16.0, 64.0 * TILE_SIZE.y)
            )),
            Velocity(Vec2::ZERO),
            Sprite {
                texture: monster_tex.clone(),
                frame: ivec2(
                    rand::gen_range(0, 1),
                    rand::gen_range(0, 7)
                )
            },
            Moving(false),
            TargetPosition(Vec2::ZERO),
            Health(rand::gen_range(50.0, 80.0)),
            Target(None)
        )});

        world.spawn_batch(monsters);

        Self {
            world,
            map: Map::new("assets/map.json", "assets/tiles.png").await,
            camera: Camera2D::from_display_rect(Rect::new(
                0.0, 0.0, screen_width(), -screen_height()
            ))
        }
    }

    pub fn events(&mut self) {
        InputSystem::keyboard_controller::<Player>(&mut self.world);
        InputSystem::mouse_controller::<Player>(&mut self.world, &self.camera);
        InputSystem::ai_controller::<Monster>(&mut self.world);
    }

    pub fn update(&mut self) {
        MovementSystem::update(&mut self.world, &mut self.map, &mut self.camera);
    }

    pub fn draw(&mut self) {
        clear_background(SKYBLUE);
        self.map.draw();

        RenderSystem::draw_entities(&mut self.world);
        RenderSystem::debug(&mut self.world, &self.camera);

        for (_, target) in self.world.query::<&Target>().with::<&Player>().iter() {
            if let Some(monster) = target.0 {
                if let Ok(pos) = self.world.get::<&Position>(monster) {
                    draw_rectangle_lines(
                        pos.0.x - TILE_OFFSET.x,
                        pos.0.y - TILE_OFFSET.y,
                        TILE_SIZE.x, TILE_SIZE.y,
                        2.0, PURPLE
                    );
                }
            }
        }

        set_camera(&self.camera);
    }
}