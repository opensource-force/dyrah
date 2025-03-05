use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

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

    pub fn is_walkable(&self, layer_name: &str, x: f32, y: f32) -> bool {
        let layer = self.get_layer(layer_name).unwrap();
        let tile_left = (x / self.tilewidth as f32).floor() as u32;
        let tile_right = ((x + self.tilewidth as f32) / self.tilewidth as f32).floor() as u32;
        let tile_top = (y / self.tileheight as f32).floor() as u32;
        let tile_bottom = ((y + self.tileheight as f32) / self.tileheight as f32).floor() as u32;

        for tile_y in tile_top..=tile_bottom {
            for tile_x in tile_left..=tile_right {
                if tile_x >= layer.width || tile_y >= layer.height {
                    return false;
                }

                let index = (tile_y * layer.width + tile_x) as usize;

                if index >= layer.data.len() {
                    return false;
                }

                if layer.data[index] != 0 {
                    return false;
                }
            }
        }

        true
    }
}
