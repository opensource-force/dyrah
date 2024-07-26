use std::{sync::mpsc::{self, Receiver}, thread};

use macroquad::{math::{IVec2, Vec2}, texture::Texture2D};
use shipyard::{Component, EntityId, Unique};

#[derive(Unique)]
pub struct Player(pub EntityId);

#[derive(Component)]
pub struct Position(pub Vec2);
#[derive(Component)]
pub struct Velocity(pub Vec2);
#[derive(Component)]
pub struct Sprite {
    pub tex: Texture2D,
    pub frame: IVec2
}

pub fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();

        std::io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer.trim_end().to_string()).unwrap();
    });
    rx
}