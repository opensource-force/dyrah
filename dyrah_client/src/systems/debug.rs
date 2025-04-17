use dyrah_shared::{Position, TILE_OFFSET, TILE_SIZE, TargetPosition};
use macroquad::{
    color::{BLACK, PURPLE},
    shapes::{draw_circle_lines, draw_line, draw_rectangle_lines},
    ui::root_ui,
};
use secs::World;

use crate::{Player, camera::Camera};

pub struct DebugSystem {}

impl DebugSystem {
    pub fn player(cam: &Camera) -> impl Fn(&World) {
        |w| {
            w.query(|_, _: &Player, pos: &Position, tgt_pos: &TargetPosition| {
                let screen_pos = cam.inner.world_to_screen(pos.vec);
                let tile_pos = (pos.vec / TILE_SIZE).floor();

                if let Some(path) = &tgt_pos.path {
                    for window in path.windows(2) {
                        let start = window[0] + TILE_OFFSET;
                        let end = window[1] + TILE_OFFSET;
                        draw_line(start.x, start.y, end.x, end.y, 1.0, BLACK);
                        draw_circle_lines(end.x, end.y, 2.0, 2.0, PURPLE);
                    }

                    for point in path {
                        draw_rectangle_lines(point.x, point.y, TILE_SIZE, TILE_SIZE, 2.0, PURPLE);
                    }
                }

                root_ui().label(
                    None,
                    &format!(
                        "Player Position: Screen({}, {}) World({}, {}) Tile({}, {})",
                        screen_pos.x, screen_pos.y, pos.vec.x, pos.vec.y, tile_pos.x, tile_pos.y
                    ),
                );
            });
        }
    }
}
