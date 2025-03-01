use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize, Debug)]
pub struct TiledMap {
    pub width: u32,
    pub height: u32,
    pub tilewidth: u32,
    pub tileheight: u32,
    pub layers: Vec<TiledLayer>
}

#[derive(Deserialize, Debug)]
pub struct TiledLayer {
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub visible: bool,
    pub data: Vec<u32>
}

impl TiledMap {
    pub fn new(path: &str) -> Self {
        let content = read_to_string(path).expect("Failed to read map file");
        from_str(&content).expect("Failed to parse JSON map")
    }

    pub fn get_layer(&self, layer_name: &str) -> Option<&TiledLayer> {
        self.layers.iter().find(|l| l.name == layer_name)
    }

    pub fn is_walkable(&self, layer_name: &str, x: u32, y: u32) -> bool {
        if let Some(layer) = self.get_layer(layer_name) {
            if x < layer.width && y < layer.height {
                let i = (y * layer.width + x) as usize;
                
                return layer.data[i] == 0;
            }
        }
        
        true
    }
}