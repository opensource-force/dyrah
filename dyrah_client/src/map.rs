use std::collections::HashMap;

use dyrah_shared::map::TiledMap;
use macroquad::{
    color::WHITE,
    math::Rect,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};
pub struct Map {
    pub tiled: TiledMap,
    textures: HashMap<u32, Texture2D>,
}

impl Map {
    pub async fn new(path: &str) -> Self {
        let tiled = TiledMap::new(path);
        let mut textures = HashMap::new();

        for tileset in &tiled.tilesets {
            if let Some(path) = &tileset.image {
                let texture = load_texture(&format!("assets/{}", path)).await.unwrap();
                textures.insert(tileset.firstgid, texture);

                println!("Loaded tileset: {}", tileset.image.as_ref().unwrap());
            }
        }

        Self { tiled, textures }
    }

    pub fn draw_tile_layer(&self, layer_name: &str) {
        let layer = self.tiled.get_layer(layer_name).unwrap();
        let (layer_w, layer_h) = (layer.width.unwrap(), layer.height.unwrap());
        let (tile_w, tile_h) = (self.tiled.tilewidth, self.tiled.tileheight);

        for y in 0..layer_h {
            for x in 0..layer_w {
                if let Some(data) = &layer.data {
                    let tile_id = data[(y * layer_w + x) as usize];

                    if tile_id <= 0 {
                        continue;
                    }

                    if let Some(tileset) = self
                        .tiled
                        .tilesets
                        .iter()
                        .filter(|set| tile_id >= set.firstgid)
                        .last()
                    {
                        let tex = self.textures.get(&tileset.firstgid).unwrap();
                        let tileset_tile_w = tileset.tilewidth.unwrap();
                        let tileset_tile_h = tileset.tileheight.unwrap();
                        let tiles_per_row = tex.width() as u32 / tileset_tile_w;
                        let local_tile_id = tile_id - tileset.firstgid;
                        let (tile_x, tile_y) = (
                            local_tile_id % tiles_per_row * tileset_tile_w,
                            local_tile_id / tiles_per_row * tileset_tile_h,
                        );

                        draw_texture_ex(
                            tex,
                            (x * tile_w) as f32,
                            (y * tile_h) as f32,
                            WHITE,
                            DrawTextureParams {
                                source: Some(Rect {
                                    x: tile_x as f32,
                                    y: tile_y as f32,
                                    w: tileset_tile_w as f32,
                                    h: tileset_tile_h as f32,
                                }),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    }

    pub fn draw_tiles(&self) {
        for layer in &self.tiled.layers {
            if !layer.visible || layer.data.is_none() {
                continue;
            }

            self.draw_tile_layer(&layer.name);
        }
    }
}
