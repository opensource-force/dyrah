use std::{ffi::OsStr, path::Path};

use macroquad::{
    file::load_string,
    math::{vec2, Rect, Vec2},
    texture::{load_texture, FilterMode}
};
use macroquad_tiled as tiled;

pub const TILE_SIZE: Vec2 = vec2(32.0, 32.0);

pub struct Tile {
    id: u32,
    pub rect: Rect,
    pub walkable: bool
}

pub struct Map {
    pub tiled: tiled::Map,
    pub chunk: Vec<Tile>
}

impl Map {
    pub async fn new(data_path: &str, tex_path: &str) -> Self {
        let tex = load_texture(tex_path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);

        let tex_basename = Path::new(tex_path).file_name();

        Self {
            tiled: tiled::load_map(
                &load_string(data_path).await.unwrap(),
                &[(
                    tex_basename.and_then(OsStr::to_str).unwrap(),
                    tex
                )], &[]
            ).unwrap(),
            chunk: Vec::new()
        }
    }

    pub fn update(&mut self, layers: &[&str], bounds: Rect) {
        self.chunk.clear();

        for layer_name in layers {
            let layer = &self.tiled.layers[*layer_name];
            
            for y in 0..layer.height {
                for x in 0..layer.width {
                    if let Some(tile) = &self.tiled.get_tile(&layer_name, x, y) {
                        let world_pos = vec2(x as f32, y as f32) * TILE_SIZE;

                        if bounds.contains(world_pos) {
                            self.chunk.push(Tile {
                                id: tile.id,
                                rect: Rect::new(
                                    world_pos.x, world_pos.y,
                                    TILE_SIZE.x, TILE_SIZE.y
                                ),
                                walkable: !(*layer_name == "colliders")
                            });
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&self) {
        for tile in &self.chunk {
            self.tiled.spr(
                "tiles",
                tile.id,
                Rect::new(
                    tile.rect.x + tile.rect.w / 2.0,
                    tile.rect.y + tile.rect.h / 2.0,
                    tile.rect.w, tile.rect.h
                )
            );
        }
    }

    pub fn get_tile(&self, x: f32, y: f32) -> Option<&Tile> {
        for tile in &self.chunk {
            if tile.rect.contains(vec2(x, y)) {
                return Some(&tile)

            }
        }

        None
    }
}