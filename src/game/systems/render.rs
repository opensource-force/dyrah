use super::*;

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
}