use super::*;
use collections::storage;
use map::{Map, TILE_SIZE};

pub struct Game {
    world: World,
}

impl Game {
    pub async fn new() -> Self {
        let mut world = World::new();
        storage::store(WorldTime(get_time()));

        let player_id = world.add_entity((
            Position(Vec2::ZERO),
            Velocity(Vec2::ZERO),
            Sprite {
                tex: load_texture("assets/32rogues/rogues.png").await.unwrap(),
                frame: ivec2(1, 4),
            },
            Moving(false),
            TargetPosition(Vec2::ZERO),
            Health(100.0),
            Damage(5.0)
        ));

        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();
        let _monster_ids = world.bulk_add_entity((0..199).map(|_| {
            (
                Monster,
                Position(vec2(
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.x),
                    rand::gen_range(0.0, 64.0 * TILE_SIZE.y),
                )),
                Velocity(Vec2::ZERO),
                Sprite {
                    tex: monster_tex.clone(),
                    frame: ivec2(rand::gen_range(0, 1), rand::gen_range(0, 7)),
                },
                Health(50.0)
            )
        }));

        world.add_unique(Map::new("assets/map.json", "assets/tiles.png").await);
        world.add_unique(Camera(Camera2D::from_display_rect(Rect::new(
            0.0, 0.0, screen_width(), -screen_height()
        ))));
        world.add_unique(Player(player_id));

        world.add_workload(Workloads::events);
        world.add_workload(Workloads::update);
        world.add_workload(Workloads::draw);

        Self { world }
    }

    pub fn events(&self) {
        self.world.run_workload(Workloads::events).unwrap();
    }

    pub fn update(&mut self) {
        self.world.run_workload(Workloads::update).unwrap();
    }

    pub fn draw(&mut self) {
        self.world.run_workload(Workloads::draw).unwrap();
    }
}