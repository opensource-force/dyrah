use super::*;
use macroquad_tiled as tiled;

pub const TILE_SIZE: Vec2 = vec2(32.0, 32.0);
pub const TILE_OFFSET: Vec2 = vec2(16.0, 16.0);

pub struct Tile {
    id: u32,
    pub rect: Rect,
    pub walkable: bool
}

pub struct Map {
    tiled: tiled::Map,
    pub chunk: Vec<Tile>
}

impl Map {
    pub async fn new(data_path: &str, tex_path: &str) -> Self {
        Self {
            tiled: tiled::load_map(
                &load_string(data_path).await.unwrap(),
                &[("tiles.png", load_texture(tex_path).await.unwrap())], &[]
            ).unwrap(),
            chunk: Vec::new()
        }
    }

    pub fn update(&mut self, bounds: Rect) {
        self.chunk.clear();

        for (layer_name, layer) in &self.tiled.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    if let Some(tile) = self.tiled.get_tile(&layer_name, x, y) {
                        let world_pos = vec2(x as f32, y as f32) * TILE_SIZE;

                        if bounds.contains(world_pos) {
                            self.chunk.push(Tile {
                                id: tile.id,
                                rect: Rect::new(
                                    world_pos.x, world_pos.y,
                                    TILE_SIZE.x, TILE_SIZE.y
                                ),
                                walkable: layer_name != "colliders"
                            })
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for tile in &self.chunk {
            self.tiled.spr("tiles", tile.id, Rect::new(
                tile.rect.x, tile.rect.y, tile.rect.w, tile.rect.h
            ))
        }
    }

    pub fn get_tile(&self, pos: Vec2) -> Option<&Tile> {
        for tile in &self.chunk {
            if tile.rect.contains(pos) {
                return Some(&tile)
            }
        }

        return None
    }
}