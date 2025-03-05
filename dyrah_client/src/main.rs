use dyrah_client::game::Game;
use macroquad::{miniquad::conf::Platform, window::Conf};

fn _window_conf() -> Conf {
    Conf {
        window_title: "Dyrah".to_owned(),
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main("Dyrah")]
async fn main() {
    let mut game = Game::new().await;

    game.run().await;
}
