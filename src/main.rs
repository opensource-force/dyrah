use macroquad::prelude::*;

mod world;
mod map;
mod entity;

use world::*;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut world = World::new().await;

    loop {
        //clear_background(WHITE);

        world.update();
        world.draw();

        next_frame().await;
    }
}