use dyrah_shared::{Health, Position, TILE_SIZE, TargetPosition};
use macroquad::{
    color::{BLACK, GREEN, RED, WHITE},
    math::Rect,
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::draw_text,
    texture::{DrawTextureParams, draw_texture_ex},
};
use secs::World;

use crate::{Creature, CreatureTexture, Damages, Player, PlayerTexture, Sprite};

pub struct RenderSystem {}

impl RenderSystem {
    pub fn player(tex: PlayerTexture) -> impl Fn(&World) {
        move |w| {
            w.query(
                |_,
                 _: &Player,
                 spr: &mut Sprite,
                 pos: &Position,
                 tgt_pos: &TargetPosition,
                 health: &Health| {
                    draw_texture_ex(
                        &tex.0,
                        pos.vec.x - spr.is_flipped.x as i8 as f32 * TILE_SIZE,
                        pos.vec.y - TILE_SIZE,
                        WHITE,
                        DrawTextureParams {
                            source: Some(spr.animation.frame().source_rect),
                            dest_size: Some(spr.animation.frame().dest_size),
                            flip_x: spr.is_flipped.x,
                            flip_y: spr.is_flipped.y,
                            ..Default::default()
                        },
                    );

                    draw_rectangle(
                        pos.vec.x,
                        pos.vec.y - TILE_SIZE,
                        health.points / 100. * TILE_SIZE,
                        4.,
                        GREEN,
                    );

                    draw_rectangle_lines(
                        tgt_pos.vec.x,
                        tgt_pos.vec.y,
                        TILE_SIZE,
                        TILE_SIZE,
                        2.,
                        WHITE,
                    );
                },
            );
        }
    }

    pub fn creature(tex: CreatureTexture) -> impl Fn(&World) {
        move |w| {
            w.query(
                |_,
                 _: &Creature,
                 spr: &Sprite,
                 pos: &Position,
                 tgt_pos: &TargetPosition,
                 health: &Health| {
                    draw_texture_ex(
                        &tex.0,
                        pos.vec.x,
                        pos.vec.y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(spr.frame.0, spr.frame.1, TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    );

                    draw_rectangle(
                        pos.vec.x,
                        pos.vec.y,
                        health.points / 100. * TILE_SIZE,
                        4.,
                        RED,
                    );

                    draw_rectangle_lines(
                        tgt_pos.vec.x,
                        tgt_pos.vec.y,
                        TILE_SIZE,
                        TILE_SIZE,
                        2.0,
                        WHITE,
                    );
                },
            );
        }
    }

    pub fn damages(damages: &Damages) -> impl Fn(&World) {
        |w| {
            for num in &damages.numbers {
                if let Some(pos) = w.get::<Position>(num.origin.into()) {
                    draw_rectangle_lines(pos.vec.x, pos.vec.y, TILE_SIZE, TILE_SIZE, 2., BLACK);

                    draw_text(
                        &num.value.to_string(),
                        num.position.x,
                        num.position.y,
                        16.,
                        RED,
                    );
                }
            }
        }
    }
}
