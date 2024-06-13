use super::*;

use macroquad_tiled as tiled;

pub struct Tile {
    id: u32,
    pub rect: Rect,
    pub walkable: bool
}

pub struct Map {
    pub tilemap: tiled::Map,
    pub chunk: Vec<Tile>
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
            chunk: Vec::new()
        }
    }

    pub fn update(&mut self, player_x: f32, player_y: f32) {
        self.chunk.clear();

        for layer in ["Base", "Props"] {
            let map_layer = &self.tilemap.layers[layer];

            for y in 0..map_layer.height {
                for x in 0..map_layer.width {
                    if let Some(tile) = &self.tilemap.get_tile(layer, x, y) {
                        let world_pos = map_to_world(vec2(x as f32, y as f32));

                        if world_pos.x < player_x + screen_width() / 2.0
                            && world_pos.x > player_x - screen_width() / 2.0 - TILE_SIZE.x
                            && world_pos.y < player_y + screen_height() / 2.0
                            && world_pos.y > player_y - screen_height() / 2.0 - TILE_SIZE.y
                        {
                            let tile = Tile {
                                id: tile.id,
                                rect: Rect::new(
                                    world_pos.x, world_pos.y,
                                    TILE_SIZE.x, TILE_SIZE.y
                                ),
                                walkable: layer == "Base"
                            };
                            self.chunk.push(tile);
                        }
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for tile in &self.chunk {
            tiled::Map::spr(
                &self.tilemap,
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
}