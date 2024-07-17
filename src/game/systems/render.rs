use super::*;
use crate::game::{Camera, Health, Player, Position, Sprite, Target, TargetPosition};
use macroquad::ui::root_ui;

pub struct RenderSystem;

impl RenderSystem {
    pub fn draw_map(mut map: UniqueViewMut<Map>) {
        map.draw();
    }

    pub fn draw_entities(
        positions: View<Position>,
        sprites: View<Sprite>,
        healths: View<Health>
    ) {
        for (pos, sprite, health) in (&positions, &sprites, &healths).iter() {
            draw_texture_ex(
                &sprite.tex,
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        (sprite.frame.x as f32) * TILE_SIZE.x,
                        (sprite.frame.y as f32) * TILE_SIZE.y,
                        TILE_SIZE.x,
                        TILE_SIZE.y,
                    )),
                    ..Default::default()
                },
            );

            draw_rectangle(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - 20.0,
                TILE_SIZE.x,
                4.0,
                DARKGRAY,
            );

            draw_rectangle(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - 20.0,
                (TILE_SIZE.x * health.0 / 50.0).clamp(0.0, TILE_SIZE.x),
                4.0,
                GREEN,
            );
        }
    }

    pub fn set_camera(camera: UniqueView<Camera>) {
        set_camera(&camera.0);
    }

    pub fn draw_player_target(
        player: UniqueView<Player>,
        position: View<Position>,
        target: View<Target>
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

    pub fn debug(
        player: UniqueView<Player>,
        positions: View<Position>,
        camera: UniqueView<Camera>,
        target_positions: View<TargetPosition>
    ) {
        let player_pos = &positions[player.0];
        let player_target_pos = &target_positions[player.0];

        let tile_pos = (player_pos.0 / TILE_SIZE).floor();
        let mouse_pos = camera.0.screen_to_world(mouse_position().into());

        root_ui().label(
            None,
            &format!(
                "Map Position: ({:.1}, {:.1})",
                player_pos.0.x, player_pos.0.y
            ),
        );
        root_ui().label(
            None,
            &format!("Tile Position: ({}, {})", tile_pos.x, tile_pos.y),
        );
        root_ui().label(
            None,
            &format!("Mouse Position: ({:.1}, {:.1})", mouse_pos.x, mouse_pos.y),
        );
        root_ui().label(
            None,
            &format!(
                "Target Position: ({}, {})",
                player_target_pos.0.x, player_target_pos.0.y
            ),
        );
        root_ui().label(None, &format!("FPS: {:.1}", get_fps()));

        draw_rectangle_lines(
            player_pos.0.x - TILE_OFFSET.x,
            player_pos.0.y - TILE_OFFSET.y,
            TILE_SIZE.x,
            TILE_SIZE.y,
            2.0,
            BLUE,
        );

        draw_rectangle_lines(
            mouse_pos.x - TILE_OFFSET.x,
            mouse_pos.y - TILE_OFFSET.y,
            TILE_SIZE.x,
            TILE_SIZE.y,
            2.0,
            GREEN,
        );

        draw_rectangle_lines(
            player_target_pos.0.x - TILE_OFFSET.x,
            player_target_pos.0.y - TILE_OFFSET.y,
            TILE_SIZE.x,
            TILE_SIZE.y,
            2.0,
            ORANGE,
        );

        for pos in (&positions).iter() {
            draw_rectangle_lines(
                pos.0.x - TILE_OFFSET.x,
                pos.0.y - TILE_OFFSET.y,
                TILE_SIZE.x,
                TILE_SIZE.y,
                2.0,
                RED,
            );
        }
    }
}
