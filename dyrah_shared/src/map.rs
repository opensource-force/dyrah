use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

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

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn get_object(&self, layer_name: &str, name: &str) -> Option<&TiledObject> {
        self.get_layer(layer_name)
            .and_then(|l| l.objects.as_ref()?.iter().find(|o| o.name == name))
    }

    fn within_bounds(&self, x: u32, y: u32) -> bool {
        if x >= self.width * self.tilewidth || y >= self.height * self.tileheight {
            return false;
        }

        true
    }

    pub fn is_walkable(&self, layer_name: &str, x: u32, y: u32) -> bool {
        if let Some(layer) = self.get_layer(layer_name) {
            if let Some((tile_x, tile_y)) = self.world_to_tile(x, y) {
                let index = tile_y * layer.width.unwrap() as usize + tile_x;

                return layer
                    .data
                    .as_ref()
                    .and_then(|data| data.get(index))
                    .map_or(false, |&tile| tile == 0);
            }
        }

        false
    }

    pub fn world_to_tile(&self, x: u32, y: u32) -> Option<(usize, usize)> {
        if !self.within_bounds(x, y) {
            return None;
        }

        Some((
            (x / self.tilewidth) as usize,
            (y / self.tileheight) as usize,
        ))
    }

    pub fn tile_to_world(&self, x: usize, y: usize) -> Option<(u32, u32)> {
        let (x, y) = (
            ((x as u32 * self.tilewidth) + self.tilewidth / 2) - self.tilewidth / 2,
            ((y as u32 * self.tileheight) + self.tileheight / 2) - self.tileheight / 2,
        );

        if !self.within_bounds(x, y) {
            return None;
        }

        Some((x, y))
    }

    pub fn get_tile(&self, layer_name: &str, x: u32, y: u32) -> Option<(usize, usize)> {
        let layer = self.get_layer(layer_name)?;

        if let Some((tile_x, tile_y)) = self.world_to_tile(x, y) {
            let index = tile_y * layer.width? as usize + tile_x;

            if layer
                .data
                .as_ref()
                .and_then(|data| data.get(index))
                .map_or(false, |&tile| tile != 0)
            {
                return Some((tile_x, tile_y));
            }
        }

        None
    }
}
