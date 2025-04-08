use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

use crate::{Vec2, vec2};

#[derive(Deserialize, Debug)]
pub struct TiledObject {
    pub id: u32,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Deserialize, Debug)]
pub struct TiledLayer {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub name: String,
    pub visible: bool,
    pub data: Option<Vec<u32>>,
    pub objects: Option<Vec<TiledObject>>,
}

#[derive(Deserialize, Debug)]
pub struct TiledTileset {
    pub firstgid: u32,
    pub source: Option<String>,
    pub image: Option<String>,
    pub tilecount: Option<u32>,
    pub tilewidth: Option<u32>,
    pub tileheight: Option<u32>,
}

#[derive(Deserialize)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
    pub tilesets: Vec<TiledTileset>,
}

impl TiledMap {
    pub fn new(path: &str) -> Self {
        let content = read_to_string(path).expect("Failed to read map file");
        from_str(&content).expect("Failed to parse JSON map")
    }

    fn within_bounds(&self, x: f32, y: f32) -> bool {
        if x < 0.
            || y < 0.
            || x >= (self.width * self.tilewidth) as f32
            || y >= (self.height * self.tileheight) as f32
        {
            return false;
        }

        true
    }

    pub fn world_to_tile(&self, vec: Vec2) -> Option<Vec2> {
        if !self.within_bounds(vec.x, vec.y) {
            return None;
        }

        let tile_pos = vec2(
            vec.x / self.tilewidth as f32,
            vec.y / self.tileheight as f32,
        );

        Some(tile_pos)
    }

    pub fn tile_to_world(&self, (x, y): (usize, usize)) -> Vec2 {
        vec2(
            (x * self.tilewidth as usize) as f32 + self.tilewidth as f32 / 2.0,
            (y * self.tileheight as usize) as f32 + self.tileheight as f32 / 2.0,
        )
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn get_object(&self, layer_name: &str, name: &str) -> Option<&TiledObject> {
        self.get_layer(layer_name)
            .and_then(|l| l.objects.as_ref().unwrap().iter().find(|o| o.name == name))
    }
}
