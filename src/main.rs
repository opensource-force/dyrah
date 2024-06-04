use macroquad::prelude::*;

mod world;
mod entity;

use world::*;

#[macroquad::main("Dyhra")]
async fn main() {
    let player_tex = load_texture("assets/pot_leaf.png").await.unwrap();
    let mut world = World::new(player_tex);

    loop {
        world.update();
        world.draw();

        next_frame().await;
    }
}