use super::*;

pub fn world_to_map(world_pos: Vec2, tile_size: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * tile_size;
    let jhat = vec2(-0.5, 0.25) * tile_size;
    let inverse = mat2(ihat, jhat).inverse();

    inverse.mul_vec2(world_pos)
}

pub fn map_to_world(map_pos: Vec2, tile_size: Vec2) -> Vec2 {
    let ihat = vec2(0.5, 0.25) * tile_size;
    let jhat = vec2(-0.5, 0.25) * tile_size;
    let transform = mat2(ihat, jhat);
    let offset = vec2(-tile_size.x / 2.0, 0.0);

    transform.mul_vec2(map_pos) + offset
}