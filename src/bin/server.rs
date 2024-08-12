use dyhra::game::server::Game;

fn main() {
    let mut game = Game::new();

    loop {
        game.update();
    }
}