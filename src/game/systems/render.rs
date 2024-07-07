use super::*;
use macroquad::ui::root_ui;

pub struct RenderSystem;

impl RenderSystem {
    pub fn draw_entities(world: &mut World) {
        for (_, (pos, sprite)) in world.query_mut::<(
            &Position, &Sprite
        )>() {
            draw_texture_ex(&sprite.texture,
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        (sprite.frame.x as f32) * TILE_SIZE.x,
                        (sprite.frame.y as f32) * TILE_SIZE.y,
                        TILE_SIZE.x, TILE_SIZE.y
                    )),
                    ..Default::default()
                }
            );
        }
    }

    pub fn debug(world: &mut World, camera: &Camera2D) {
        for (_, (pos, target)) in world.query_mut::<(
            &Position, &TargetPosition
        )>().with::<&Player>() {
            let tile_pos = (pos.0 / TILE_SIZE).floor();
            let mouse_pos = camera.screen_to_world(mouse_position().into());

            root_ui().label(None, &format!("Map Position: ({:.1}, {:.1})", pos.0.x, pos.0.y));    
            root_ui().label(None, &format!("Tile Position: ({}, {})", tile_pos.x, tile_pos.y));
            root_ui().label(None, &format!("Mouse Position: ({:.1}, {:.1})", mouse_pos.x, mouse_pos.y));
            root_ui().label(None, &format!("Target Position: ({}, {})", target.0.x, target.0.y));
            root_ui().label(None, &format!("FPS: {:.1}", get_fps()));

            draw_rectangle_lines(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                TILE_SIZE.x, TILE_SIZE.y,
                2.0, BLUE
            );

            draw_rectangle_lines(
                mouse_pos.x - TILE_OFFSET.x,
                mouse_pos.y - TILE_OFFSET.y,
                TILE_SIZE.x, TILE_SIZE.y,
                2.0, GREEN
            );

            draw_rectangle_lines(
                target.0.x - TILE_OFFSET.x,
                target.0.y - TILE_OFFSET.y,
                TILE_SIZE.x, TILE_SIZE.y,
                2.0, ORANGE
            );
        }

        for (_, pos) in world.query_mut::<&Position>().with::<&Monster>() {
            draw_rectangle_lines(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                TILE_SIZE.x, TILE_SIZE.y,
                2.0, RED
            );
        }
    }
}