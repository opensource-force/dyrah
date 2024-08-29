use dyhra::game::client::Game;
use macroquad::{
    color::SKYBLUE,
    miniquad::conf::Platform,
    window::{clear_background, next_frame, Conf},
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Dyhra".to_owned(),
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(SKYBLUE);

        game.update();
        game.draw();

        next_frame().await;
    }
}
