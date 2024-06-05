use super::*;

use macroquad_tiled as tiled;

const TILE_SIZE: IVec2 = ivec2(32, 32);

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
        self.draw_isometric_tiles("Base");
        self.draw_isometric_tiles("Props");
    }

    fn draw_isometric_tiles(&self, layer: &str) {
        let map_layer = &self.tilemap.layers[layer];
    
        for y in 0..map_layer.height {
            for x in 0..map_layer.width {
                if let Some(tile) = self.tilemap.get_tile(layer, x, y) {
                    let world_pos = map_to_world(ivec2(x as i32, y as i32));

                    tiled::Map::spr(
                        &self.tilemap,
                        "tileset",
                        tile.id,
                        Rect::new(world_pos.x, world_pos.y, 32.0, 32.0)
                    );
                }
            }
        }
    }
}

fn world_to_map(world_pos: Vec2) -> IVec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE.as_vec2();
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE.as_vec2();
    let inverse = mat2(ihat, jhat).inverse();

    inverse.mul_vec2(world_pos).as_ivec2()
}

fn map_to_world(map_pos: IVec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * TILE_SIZE.as_vec2();
    let jhat = vec2(-0.5, 0.25) * TILE_SIZE.as_vec2();
    let transform = mat2(ihat, jhat);
    let offset = ivec2(-TILE_SIZE.x / 2, 0);

    transform.mul_vec2(map_pos.as_vec2()) + offset.as_vec2()
}