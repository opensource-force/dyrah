use dyrah::game::server::Game;


fn main() {
    let mut game = Game::new();

    game.run(60);
}