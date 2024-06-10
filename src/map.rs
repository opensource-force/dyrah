use macroquad_tiled as tiled;

use super::*;

const TILE_WIDTH: f32 = 32.0;
const TILE_HEIGHT: f32 = 32.0;
const TILE_SIZE: Vec2 = vec2(TILE_WIDTH, TILE_HEIGHT);

pub struct Map {
    pub tilemap: tiled::Map,
    pub colliders: Vec<Rect>
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
            colliders: Vec::new()
        }
    }

    pub fn draw(&mut self) {
        self.draw_isometric_tiles("Base", false);
        self.draw_isometric_tiles("Props", true);
    }

    fn draw_isometric_tiles(&mut self, layer: &str, collision: bool) {
        let map_layer = &self.tilemap.layers[layer];
    
        for y in 0..map_layer.height {
            for x in 0..map_layer.width {
                if let Some(tile) = &self.tilemap.get_tile(layer, x, y) {
                    let world_pos = map_to_world(vec2(x as f32, y as f32));
    
                    tiled::Map::spr(
                        &self.tilemap,
                        "tileset",
                        tile.id,
                        Rect::new(
                            world_pos.x, world_pos.y,
                            TILE_WIDTH, TILE_HEIGHT
                        )
                    );

                    if collision {
                        self.colliders.push(
                            Rect::new(
                                world_pos.x, world_pos.y,
                                TILE_WIDTH, TILE_HEIGHT
                            )
                        );

                        draw_rectangle_lines(
                            world_pos.x + 16.0, world_pos.y + 16.0,
                            TILE_WIDTH, TILE_HEIGHT, 2.0,
                            RED
                        );
                    }
                }
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