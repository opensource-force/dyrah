use dyhra::game::client::Game;
use macroquad::{color::SKYBLUE, window::{clear_background, next_frame}};

#[macroquad::main("Dyhra")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(SKYBLUE);

        game.update();
        game.draw();

        next_frame().await;
    }
}