use dyrah_shared::map::TiledMap as Inner;
use macroquad::{
    color::WHITE,
    math::Rect,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
};
pub struct TiledMap {
    tiled_map: Inner,
}

impl TiledMap {
    pub fn new(path: &str) -> Self {
        Self {
            tiled_map: Inner::new(path),
        }
    }

    pub fn draw_tile_layer(&self, layer_name: &str, tex: &Texture2D) {
        let layer = self.tiled_map.get_layer(layer_name).unwrap();
        let tile_height = self.tiled_map.tileheight;
        let tile_width = self.tiled_map.tilewidth;
        let tiles_per_row = (tex.width() / tile_width as f32) as u32;

        for y in 0..layer.height {
            for x in 0..layer.width {
                let mut tile_id = layer.data[(y * layer.width + x) as usize];

                if tile_id <= 0 {
                    continue;
                }

                tile_id -= 1;

                let tile_x = tile_id % tiles_per_row * tile_width;
                let tile_y = tile_id / tiles_per_row * tile_height;

                draw_texture_ex(
                    tex,
                    (x * tile_width) as f32,
                    (y * tile_height) as f32,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect {
                            x: tile_x as f32,
                            y: tile_y as f32,
                            w: tile_width as f32,
                            h: tile_height as f32,
                        }),
                        ..Default::default()
                    },
                );
            }
        }
    }

    pub fn draw_tiles(&self, tex: &Texture2D) {
        for layer in self.tiled_map.layers.iter() {
            if !layer.visible {
                continue;
            }

            self.draw_tile_layer(&layer.name, tex);
        }
    }
}
