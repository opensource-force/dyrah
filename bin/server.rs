use dyrah_server::game::Game;

fn main() {
    let mut game = Game::new();

    game.run(60);
}