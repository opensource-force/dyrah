use super::*;
use macroquad::ui::root_ui;
use crate::game::{Player, Position, Sprite, Camera};

pub struct RenderSystem;

impl RenderSystem {
    pub fn draw_entities(
        player: UniqueView<Player>,
        positions: View<Position>,
        sprites: View<Sprite>
    ) {
        draw_texture_ex(
            &player.spr.tex,
            player.pos.0.x - TILE_OFFSET.x,
            player.pos.0.y - TILE_OFFSET.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    (player.spr.frame.x as f32) * TILE_SIZE.x,
                    (player.spr.frame.y as f32) * TILE_SIZE.y,
                    TILE_SIZE.x, TILE_SIZE.y
                )),
                ..Default::default()
            }
        );

        for (pos, sprite) in (&positions, &sprites).iter() {
            draw_texture_ex(
                &sprite.tex,
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        (sprite.frame.x as f32) * TILE_SIZE.x,
                        (sprite.frame.y as f32) * TILE_SIZE.y,
                        TILE_SIZE.x, TILE_SIZE.y,
                    )),
                    ..Default::default()
                },
            );
        }
    }

    pub fn debug(player: UniqueView<Player>, positions: View<Position>, camera: UniqueView<Camera>) {
        let tile_pos = (player.pos.0 / TILE_SIZE).floor();
        let mouse_pos = camera.0.screen_to_world(mouse_position().into());

        root_ui().label(None, &format!("Map Position: ({:.1}, {:.1})", player.pos.0.x, player.pos.0.y));
        root_ui().label(None, &format!("Tile Position: ({}, {})", tile_pos.x, tile_pos.y));
        root_ui().label(None, &format!("Mouse Position: ({:.1}, {:.1})", mouse_pos.x, mouse_pos.y));
        root_ui().label(None, &format!("Target Position: ({}, {})", player.target_pos.0.x, player.target_pos.0.y));
        root_ui().label(None, &format!("FPS: {:.1}", get_fps()));

        draw_rectangle_lines(
            player.pos.0.x - TILE_OFFSET.x,
            player.pos.0.y - TILE_OFFSET.y,
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
            player.target_pos.0.x - TILE_OFFSET.x,
            player.target_pos.0.y - TILE_OFFSET.y,
            TILE_SIZE.x, TILE_SIZE.y,
            2.0, ORANGE
        );

        for pos in (&positions).iter() {
            draw_rectangle_lines(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                TILE_SIZE.x, TILE_SIZE.y,
                2.0, RED
            );
        }
    }
}