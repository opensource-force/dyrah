mod components;
mod game;
mod map;

use std::{
    thread,
    time::{Duration, Instant},
};

use crate::game::Game;

fn main() {
    let mut game = Game::new();
    let frame_time = Duration::from_millis(33); // ~30 FPS
    let mut last = Instant::now();

    loop {
        let now = Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        game.handle_events();
        game.update(dt);

        let elapsed = now.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }
}
