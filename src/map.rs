use super::*;

use macroquad_tiled as tiled;

const TILE_SIZE: Vec2 = vec2(32.0, 32.0);

pub struct Tile {
    id: u32,
    pub rect: Rect,
    pub walkable: bool
}

pub struct Map {
    pub tilemap: tiled::Map,
    pub tiles: Vec<Tile>
}

impl Map {
    pub async fn new(tex_path: &str, data_path: &str) -> Self {
        let tex = load_texture(tex_path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);

        Self {
            tilemap: tiled::load_map(
                &load_string(data_path).await.unwrap(),
                &[("tileset.png", tex)], &[]
            ).unwrap(),
            tiles: Vec::new()
        }
    }

    pub fn update(&mut self) {
        self.tiles.clear();

        for layer in ["Base", "Props"] {
            let map_layer = &self.tilemap.layers[layer];

            for y in 0..map_layer.height {
                for x in 0..map_layer.width {
                    if let Some(tile) = &self.tilemap.get_tile(layer, x, y) {
                        let world_pos = map_to_world(vec2(x as f32, y as f32));

                        let tile = Tile {
                            id: tile.id,
                            rect: Rect::new(
                                world_pos.x, world_pos.y,
                                TILE_SIZE.x, TILE_SIZE.y
                            ),
                            walkable: layer == "Base"
                        };
                        self.tiles.push(tile);
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for tile in &self.tiles {
            let tile_offset = Rect::new(
                tile.rect.x + 16.0, tile.rect.y + 16.0,
                tile.rect.w, tile.rect.h
            );
            
            tiled::Map::spr(
                &self.tilemap,
                "tileset",
                tile.id,
                tile_offset
            );

            if !tile.walkable {
                draw_rectangle_lines(
                    tile_offset.x, tile_offset.y,
                    tile_offset.w, tile_offset.h,
                    2.0, RED
                );
            }
        }
    }
}

pub fn world_to_map(world_pos: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE;
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE;
    let inverse = mat2(ihat, jhat).inverse();

    inverse.mul_vec2(world_pos)
}

pub fn map_to_world(map_pos: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE;
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE;
    let transform = mat2(ihat, jhat);
    let offset = vec2(-TILE_SIZE.x / 2.0, 0.0);

    transform.mul_vec2(map_pos) + offset
}