mod engine;
mod world;

use macroquad::prelude::next_frame;
use world::prelude::World;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut world = World::new().await;

    loop {
        world.update();
        world.draw();

        next_frame().await;
    }
}