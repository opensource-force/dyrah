use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_OFFSET: f32 = 16.;

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

    fn world_to_tile(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if !self.within_bounds(x, y) {
            return None;
        }

        let tile_x = (x / self.tilewidth as f32).floor() as usize;
        let tile_y = (y / self.tileheight as f32).floor() as usize;

        Some((tile_x, tile_y))
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn is_walkable(&self, layer_name: &str, x: f32, y: f32) -> bool {
        let layer = self.get_layer(layer_name).unwrap();

        if let Some((tile_x, tile_y)) = self.world_to_tile(x, y) {
            let index = tile_y * layer.width as usize + tile_x;
            return layer.data.get(index).map_or(false, |&tile| tile == 0);
        }

        false
    }

    pub fn get_walkable_tile(&self, layer_name: &str, x: f32, y: f32) -> Option<(usize, usize)> {
        let layer = self.get_layer(layer_name)?;

        if let Some((tile_x, tile_y)) = self.world_to_tile(x, y) {
            let index = tile_y * layer.width as usize + tile_x;

            if layer.data.get(index).map_or(false, |&tile| tile == 0) {
                return Some((tile_x, tile_y));
            }
        }

        None
    }

    pub fn get_tile_center(&self, layer_name: &str, x: f32, y: f32) -> Option<(f32, f32)> {
        if let Some((tile_x, tile_y)) = self.get_walkable_tile(layer_name, x, y) {
            let center_x = tile_x as u32 * self.tilewidth + (self.tilewidth / 2);
            let center_y = tile_y as u32 * self.tileheight + (self.tileheight / 2);

            return Some((center_x as f32 - TILE_OFFSET, center_y as f32 - TILE_OFFSET));
        }

        None
    }
}
