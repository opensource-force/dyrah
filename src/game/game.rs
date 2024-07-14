use super::*;
use systems::prelude::*;
use map::{Map, TILE_SIZE};

pub struct Game {
    world: World,
    map: Map
}

impl Game {
    pub async fn new() -> Self {
        let mut world = World::new();
        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

        let _player_id = world.add_unique(Player {
            pos: Position(vec2(0.0, 0.0)),
            vel: Velocity(Vec2::ZERO),
            spr: Sprite {
                tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                frame: ivec2(1, 4)
            }
        });
        let _monster_ids = world.bulk_add_entity((0..999).map(|_| (
            Monster,
            Position(vec2(
                rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                rand::gen_range(0.0, 64.0 * TILE_SIZE.y)
            )),
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
    }

    pub fn update(&mut self) {
        self.world.run(MovementSystem::update);

        if let Ok(player) = self.world.get_unique::<&Player>() {
            let mut camera = self.world.get_unique::<&mut Camera>().unwrap();
            camera.0.target = player.pos.0;
            self.map.update(Rect::new(
                player.pos.0.x - screen_width() / 2.0 - TILE_SIZE.x,
                player.pos.0.y - screen_height() / 2.0 - TILE_SIZE.y,
                screen_width() + TILE_SIZE.x,
                screen_height() + TILE_SIZE.y
            ));
        }
    }

    pub fn draw(&mut self) {
        clear_background(SKYBLUE);

        self.map.draw();
        self.world.run(RenderSystem::draw_entities);

        if let Ok(camera) = self.world.get_unique::<&Camera>() {
            set_camera(&camera.0);
        }
    }
}