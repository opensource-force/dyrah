use super::*;
use macroquad::ui::root_ui;

pub struct UiSystem;

impl UiSystem {
    pub fn statistics(world: &mut World) {
        for (_, pos) in world.query_mut::<&Position>().with::<&Player>() {
            root_ui().label(None, &format!("World Position: ({:.1}, {:.1})", pos.0.x, pos.0.y));
    
            let tile_pos = (pos.0 / TILE_SIZE).floor();
            root_ui().label(None, &format!("Tile Position: ({}, {})", tile_pos.x, tile_pos.y));
            root_ui().label(None, &format!("FPS: {:.1}", get_fps()));
        }
    }
}