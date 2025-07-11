mod components;
mod game;
mod map;
mod sprite;

use egor::app::App;

use crate::game::Game;

fn main() {
    App::init(Game::new(), |game, ctx| {
        ctx.set_title("Dyrah");

        game.load(ctx);
    })
    .run(move |game, ctx| {
        game.handle_events();
        game.update(ctx);
        game.render(ctx);
    });
}
