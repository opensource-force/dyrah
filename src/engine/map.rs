use super::*;
use std::{ffi::OsStr, path::Path};
use isometric::map_to_world;
use macroquad_tiled as tiled;

pub struct Tile {
    id: u32,
    pub rect: Rect,
    pub walkable: bool
}

pub struct Map {
    pub tiled: tiled::Map,
    perspective: bool,
    pub chunk: Vec<Tile>
}

impl Map {
    pub async fn new(data_path: &str, tex_path: &str, perspective: bool) -> Self {
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
            perspective,
            chunk: Vec::new()
        }
    }

    pub fn update(&mut self, layers: &[&str], bounds: Rect, tile_size: Vec2) {
        self.chunk.clear();

        for layer_name in layers {
            let layer = &self.tiled.layers[*layer_name];
            
            for y in 0..layer.height {
                for x in 0..layer.width {
                    if let Some(tile) = &self.tiled.get_tile(&layer_name, x, y) {
                        let world_pos = if self.perspective {
                            map_to_world(vec2(x as f32, y as f32), tile_size)
                        } else {
                            vec2(x as f32, y as f32) * tile_size
                        };

                        if bounds.contains(world_pos) {
                            self.chunk.push(Tile {
                                id: tile.id,
                                rect: Rect::new(
                                    world_pos.x, world_pos.y,
                                    tile_size.x, tile_size.y
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
                "tileset",
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
            if x >= tile.rect.x && x < tile.rect.x + tile.rect.w &&
               y >= tile.rect.y && y < tile.rect.y + tile.rect.h {
                return Some(&tile)
            }
        }

        return None
    }
}