use dyhra::game::prelude::Game;
use macroquad::prelude::*;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.events();
        game.update();
        game.draw();
        next_frame().await;
    }
}
