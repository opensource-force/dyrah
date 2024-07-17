use super::*;
use collections::storage;
use map::{Map, TILE_OFFSET, TILE_SIZE};
use systems::prelude::*;

pub struct Game {
    world: World,
}

impl Game {
    pub async fn new() -> Self {
        let mut world = World::new();
        storage::store(WorldTime(get_time()));
        let monster_tex = load_texture("assets/32rogues/monsters.png").await.unwrap();

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
            Damage(5.0),
        ));

        world.add_unique(Player(player_id));

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
                Health(50.0),
            )
        }));

        world.add_unique(Camera(Camera2D::from_display_rect(Rect::new(
            0.0,
            0.0,
            screen_width(),
            -screen_height(),
        ))));

        world.add_unique(Map::new("assets/map.json", "assets/tiles.png").await);

        world.add_workload(events_workload);
        world.add_workload(update_workload);
        world.add_workload(draw_workload);

        Self { world }
    }

    pub fn events(&self) {
        self.world.run_workload(events_workload).unwrap();
    }

    pub fn update(&mut self) {
        self.world.run_workload(update_workload).unwrap();
    }

    pub fn draw(&mut self) {
        self.world.run_workload(draw_workload).unwrap();
    }
}

fn events_workload() -> Workload {
    (InputSystem::control_player, AiSystem::control_monsters).into_workload()
}

fn update_workload() -> Workload {
    fn move_player(
        player: UniqueView<Player>,
        mut map: UniqueViewMut<Map>,
        mut camera: UniqueViewMut<Camera>,
        mut moving: ViewMut<Moving>,
        mut target_pos: ViewMut<TargetPosition>,
        pos: View<Position>,
    ) {
        if let Some(tile) = map.get_tile(target_pos[player.0].0) {
            if tile.walkable {
                moving[player.0].0 = true;
                target_pos[player.0].0 = tile.rect.center();
            }
        } else {
            moving[player.0].0 = false;
        }

        camera.0.target = pos[player.0].0;

        map.update(Rect::new(
            pos[player.0].0.x - screen_width() / 2.0 - TILE_SIZE.x,
            pos[player.0].0.y - screen_height() / 2.0 - TILE_SIZE.y,
            screen_width() + TILE_SIZE.x,
            screen_height() + TILE_SIZE.y,
        ));
    }

    fn damage_target(
        player: UniqueView<Player>,
        mut health: ViewMut<Health>,
        target: View<Target>,
        damage: View<Damage>,
    ) {
        if let Ok(target) = target.get(player.0) {
            let monster_health = &mut health[target.0];
            monster_health.0 -= damage[player.0].0;
        }
    }

    (move_player, MovementSystem::update, damage_target).into_workload()
}

fn draw_workload() -> Workload {
    fn draw_player_target(
        player: UniqueView<Player>,
        position: View<Position>,
        target: View<Target>,
    ) {
        if let Ok(target) = target.get(player.0) {
            let monster_pos = &position[target.0];

            draw_rectangle_lines(
                monster_pos.0.x - TILE_OFFSET.x,
                monster_pos.0.y - TILE_OFFSET.y,
                TILE_SIZE.x,
                TILE_SIZE.y,
                2.0,
                PURPLE,
            );
        }
    }

    fn set_camera_sys(camera: UniqueView<Camera>) {
        set_camera(&camera.0);
    }

    fn draw_map(mut map: UniqueViewMut<Map>) {
        map.draw();
    }

    (
        |_: AllStoragesViewMut| clear_background(SKYBLUE),
        draw_map,
        RenderSystem::draw_entities,
        set_camera_sys,
        RenderSystem::debug,
        draw_player_target,
    )
        .into_workload()
}
