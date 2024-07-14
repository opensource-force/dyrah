use super::*;
use crate::game::{Player, Position, Sprite};

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
}