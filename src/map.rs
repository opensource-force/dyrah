use super::*;

use macroquad_tiled as tiled;

const TILE_SIZE: Vec2 = vec2(64.0, 64.0);

pub struct Map {
    tilemap: tiled::Map
}

impl Map {
    pub async fn new(tex_path: &str, data_path: &str) -> Self {
        let tex = load_texture(tex_path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);

        let data = load_string(data_path).await.unwrap();
        let tilemap = tiled::load_map(
            &data, &[("tileset.png", tex.clone())],
            &[]
        ).unwrap();

        Self { tilemap }
    }

    pub fn draw(&self) {
        draw_isometric_tiles(&self.tilemap, "Base");
        draw_isometric_tiles(&self.tilemap, "Props");
    }
}

fn draw_isometric_tiles(tilemap: &tiled::Map, layer: &str) {
    let map_layer = &tilemap.layers[layer];

    for y in 0..map_layer.height {
        for x in 0..map_layer.width {
            if let Some(tile) = tilemap.get_tile(layer, x, y) {
                let world_pos = map_to_world(vec2(x as f32, y as f32));

                tiled::Map::spr(
                    &tilemap,
                    "tileset",
                    tile.id,
                    Rect::new(world_pos.x, world_pos.y, 64.0, 64.0)
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