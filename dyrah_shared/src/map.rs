use std::fs::read_to_string;

use glam::{Vec2, vec2};
use serde::Deserialize;
use serde_json::from_str;

use crate::TILE_OFFSET;

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

#[derive(Deserialize, Debug)]
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

    fn world_to_tile(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        if !self.within_bounds(x, y) {
            return None;
        }

        let tile_x = x / self.tilewidth as f32;
        let tile_y = y / self.tileheight as f32;

        Some((tile_x, tile_y))
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn is_walkable(&self, layer_name: &str, vec: Vec2) -> bool {
        if let Some(layer) = self.get_layer(layer_name) {
            if let Some((tile_x, tile_y)) = self.world_to_tile(vec.x, vec.y) {
                let index = (tile_y * layer.width.unwrap() as f32 + tile_x) as usize;

                return layer
                    .data
                    .as_ref()
                    .and_then(|data| data.get(index))
                    .map_or(false, |&tile| tile == 0);
            }
        }

        false
    }

    pub fn get_tile(&self, layer_name: &str, x: f32, y: f32) -> Option<(f32, f32)> {
        let layer = self.get_layer(layer_name)?;

        if let Some((tile_x, tile_y)) = self.world_to_tile(x, y) {
            let index = (tile_y * layer.width.unwrap() as f32 + tile_x) as usize;

            if layer
                .data
                .as_ref()
                .and_then(|data| data.get(index))
                .map_or(false, |&tile| tile != 0)
            {
                return Some((tile_x as f32, tile_y as f32));
            }
        }

        None
    }

    pub fn get_tile_center(&self, layer_name: &str, vec: Vec2) -> Option<Vec2> {
        if let Some((tile_x, tile_y)) = self.get_tile(layer_name, vec.x, vec.y) {
            let center_x = tile_x as u32 * self.tilewidth + (self.tilewidth / 2);
            let center_y = tile_y as u32 * self.tileheight + (self.tileheight / 2);

            return Some(Vec2::new(
                center_x as f32 - TILE_OFFSET,
                center_y as f32 - TILE_OFFSET,
            ));
        }

        None
    }

    pub fn get_object(&self, layer_name: &str, name: &str) -> Option<&TiledObject> {
        self.get_layer(layer_name)
            .and_then(|l| l.objects.as_ref().unwrap().iter().find(|o| o.name == name))
    }

    pub fn get_spawn(&self, name: &str) -> Option<Vec2> {
        self.get_object("spawns", name).map(|o| vec2(o.x, o.y))
    }
}
