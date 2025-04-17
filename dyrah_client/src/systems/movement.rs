use dyrah_shared::{Position, TargetPosition};
use macroquad::{
    time::get_frame_time,
    window::{screen_height, screen_width},
};
use secs::World;

use crate::{Creature, Player, Sprite, camera::Camera};

pub struct MovementSystem {}

impl MovementSystem {
    pub fn player(cam: &mut Camera) -> impl FnMut(&World) {
        |w| {
            w.query(
                |_, _: &Player, spr: &mut Sprite, pos: &mut Position, tgt_pos: &TargetPosition| {
                    pos.vec = pos.vec.move_towards(tgt_pos.vec, 200.0 * get_frame_time());

                    if tgt_pos.vec.x < pos.vec.x {
                        spr.is_flipped.x = true;
                    } else if tgt_pos.vec.x > pos.vec.x {
                        spr.is_flipped.x = false;
                    }

                    spr.animation.update();
                    cam.attach_sized(pos.vec.x, pos.vec.y, screen_width(), screen_height());
                    cam.set();
                },
            );
        }
    }

    pub fn creature() -> impl Fn(&World) {
        |w| {
            w.query(
                |_, _: &Creature, pos: &mut Position, tgt_pos: &TargetPosition| {
                    pos.vec = pos.vec.move_towards(tgt_pos.vec, 150.0 * get_frame_time());
                },
            );
        }
    }
}
