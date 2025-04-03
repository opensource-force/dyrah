use std::collections::HashMap;

use dyrah_shared::map::TiledMap as Inner;
use macroquad::{
    color::WHITE,
    math::Rect,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};
pub struct TiledMap {
    tiled_map: Inner,
    textures: HashMap<u32, Texture2D>,
}

impl TiledMap {
    pub async fn new(path: &str) -> Self {
        let tiled_map = Inner::new(path);
        let mut textures = HashMap::new();

        for tileset in &tiled_map.tilesets {
            if let Some(path) = &tileset.image {
                let texture = load_texture(&format!("assets/{}", path)).await.unwrap();
                textures.insert(tileset.firstgid, texture);
            }
        }

        Self {
            tiled_map,
            textures,
        }
    }

    pub fn draw_tile_layer(&self, layer_name: &str) {
        let layer = self.tiled_map.get_layer(layer_name).unwrap();
        let (layer_w, layer_h) = (layer.width.unwrap(), layer.height.unwrap());
        let (tile_w, tile_h) = (self.tiled_map.tilewidth, self.tiled_map.tileheight);

        for y in 0..layer_h {
            for x in 0..layer_w {
                if let Some(data) = &layer.data {
                    let tile_id = data[(y * layer_w + x) as usize];

                    if tile_id <= 0 {
                        continue;
                    }

                    for (&firstgid, tex) in &self.textures {
                        if tile_id >= firstgid {
                            let tiles_per_row = (tex.width() / tile_w as f32) as u32;
                            let local_tile_id = tile_id - firstgid;

                            let (tile_x, tile_y) = (
                                local_tile_id % tiles_per_row * tile_w,
                                local_tile_id / tiles_per_row * tile_h,
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
                                        w: tile_w as f32,
                                        h: tile_h as f32,
                                    }),
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn draw_tiles(&self) {
        for layer in self.tiled_map.layers.iter() {
            if !layer.visible || layer.data.is_none() {
                continue;
            }

            self.draw_tile_layer(&layer.name);
        }
    }
}
