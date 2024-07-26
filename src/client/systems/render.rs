use super::*;
use macroquad::prelude::*;

use crate::{Position, Sprite};

const TILE_SIZE: Vec2 = vec2(32.0, 32.0);
const TILE_OFFSET: Vec2 = vec2(16.0, 16.0);

pub struct RenderSystem;

impl RenderSystem {
    pub fn draw_entities(
        positions: View<Position>,
        sprites: View<Sprite>
    ) {
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
                        TILE_SIZE.x,
                        TILE_SIZE.y,
                    )),
                    ..Default::default()
                },
            );
        }
    }
}