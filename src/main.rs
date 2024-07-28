use macroquad::prelude::next_frame;
//use futures::executor::block_on;
use dyhra::game::prelude::*;

#[macroquad::main("Dyhra")]
async fn main() {
    let mut server = Server::new("127.0.0.1:6667");

    let mut client = Client::new("127.0.0.1:6668", "127.0.0.1:6667");

    let mut buf = Vec::new();

    loop {

        server.update();
        server.handle_events();
        server.handle_messages(&mut buf);

        client.update();
        client.handle_messages();
    }
}
