use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

pub const TILE_SIZE: f32 = 32.;

#[derive(Deserialize, Debug)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>,
}

#[derive(Deserialize, Debug)]
pub struct TiledLayer {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub visible: bool,
    pub data: Vec<u32>,
}

impl TiledMap {
    pub fn new(path: &str) -> Self {
        let content = read_to_string(path).expect("Failed to read map file");
        from_str(&content).expect("Failed to parse JSON map")
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn get_tile_center(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        if x < 0.
            || y < 0.
            || x >= (self.width * self.tilewidth) as f32
            || y >= (self.height * self.tileheight) as f32
        {
            return None;
        }

        let tile_x = (x / self.tilewidth as f32).floor();
        let tile_y = (y / self.tileheight as f32).floor();
        let center_x = tile_x * self.tilewidth as f32 + (self.tilewidth as f32 / 2.);
        let center_y = tile_y * self.tileheight as f32 + (self.tileheight as f32 / 2.);

        Some((center_x, center_y))
    }
}
