use super::*;
use map::{Map, TILE_SIZE};
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
            Position(vec2(16.0, 16.0)),
            Velocity(Vec2::ZERO),
            Sprite {
                texture: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                frame: ivec2(1, 4)
            },
            Moving(false),
            TargetPosition(Vec2::ZERO)
        ));
        
        let monsters = (0..99).map(|_| {(
            Monster,
            Position(vec2(
                rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                rand::gen_range(0.0, 64.0 * TILE_SIZE.y)
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
            TargetPosition(Vec2::ZERO)
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
        InputSystem::handle_player(&mut self.world);
        InputSystem::handle_monsters(&mut self.world);
        InputSystem::update(&mut self.world, &self.map);
    }

    pub fn update(&mut self) {
        clear_background(SKYBLUE);
        
        MovementSystem::handle_player(&mut self.world, &mut self.map, &mut self.camera);
        MovementSystem::update(&mut self.world);
    }

    pub fn draw(&mut self) {
        self.map.draw();

        RenderSystem::draw_entities(&mut self.world);
        UiSystem::statistics(&mut self.world);

        set_camera(&self.camera);
    }
}