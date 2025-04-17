use dyrah_shared::{Position, TILE_OFFSET, Vec2, map::TiledMap, vec2};
use pathfinding::prelude::astar;
use secs::World;

use crate::Collider;

pub struct CollisionGrid {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl CollisionGrid {
    pub fn new(map: &Map) -> Self {
        let width = map.tiled.width as usize;
        let height = map.tiled.height as usize;
        Self {
            width,
            height,
            grid: vec![false; width * height],
        }
    }

    pub fn update(&mut self, map: &Map, world: &World) {
        self.grid.fill(false);

        for y in 0..self.height {
            for x in 0..self.width {
                let world_pos = map.tiled.tile_to_world(x, y);
                if !map.is_walkable("colliders", world_pos) {
                    self.grid[y * self.width + x] = true;
                }
            }
        }

        world.query(|_, _: &Collider, pos: &Position| {
            if let Some(tile_pos) = map.tiled.world_to_tile(pos.vec) {
                let (x, y) = (tile_pos.x as usize, tile_pos.y as usize);

                if x < self.width && y < self.height {
                    self.grid[y * self.width + x] = true;
                }
            }
        });
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            !self.grid[y * self.width + x]
        }
    }
}

pub struct Map {
    pub tiled: TiledMap,
}

impl Map {
    pub fn new(path: &str) -> Self {
        Self {
            tiled: TiledMap::new(path),
        }
    }

    pub fn is_walkable(&self, layer_name: &str, vec: Vec2) -> bool {
        if let Some(layer) = self.tiled.get_layer(layer_name) {
            if let Some(tile_pos) = self.tiled.world_to_tile(vec) {
                let index = (tile_pos.y * layer.width.unwrap() as f32 + tile_pos.x) as usize;

                return layer
                    .data
                    .as_ref()
                    .and_then(|data| data.get(index))
                    .map_or(false, |&tile| tile == 0);
            }
        }

        false
    }

    pub fn get_tile(&self, layer_name: &str, vec: Vec2) -> Option<Vec2> {
        let layer = self.tiled.get_layer(layer_name)?;

        if let Some(tile_pos) = self.tiled.world_to_tile(vec) {
            let index = (tile_pos.y * layer.width.unwrap() as f32 + tile_pos.x) as usize;

            if layer
                .data
                .as_ref()
                .and_then(|data| data.get(index))
                .map_or(false, |&tile| tile != 0)
            {
                return Some(tile_pos);
            }
        }

        None
    }

    pub fn get_tile_center(&self, layer_name: &str, vec: Vec2) -> Option<Vec2> {
        if let Some(tile_pos) = self.get_tile(layer_name, vec) {
            let center_x = tile_pos.x as u32 * self.tiled.tilewidth + (self.tiled.tilewidth / 2);
            let center_y = tile_pos.y as u32 * self.tiled.tileheight + (self.tiled.tileheight / 2);

            return Some(vec2(
                center_x as f32 - TILE_OFFSET,
                center_y as f32 - TILE_OFFSET,
            ));
        }

        None
    }

    pub fn get_spawn(&self, name: &str) -> Option<Vec2> {
        self.tiled
            .get_object("spawns", name)
            .map(|o| vec2(o.x, o.y))
    }

    fn manhattan_distance(&self, a: (usize, usize), b: (usize, usize)) -> u32 {
        ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
    }

    fn get_walkable_successors(
        &self,
        x: usize,
        y: usize,
        grid: &CollisionGrid,
    ) -> Vec<((usize, usize), u32)> {
        let mut successors = Vec::new();
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            if nx >= 0 && ny >= 0 && nx < self.tiled.width as i32 && ny < self.tiled.height as i32 {
                let (nx, ny) = (nx as usize, ny as usize);
                if grid.is_walkable(nx, ny) {
                    successors.push(((nx, ny), 1));
                }
            }
        }
        successors
    }

    pub fn find_path(
        &self,
        start: Vec2,
        end: Vec2,
        collision_grid: &CollisionGrid,
    ) -> Option<Vec<Vec2>> {
        let start_tile = self.tiled.world_to_tile(start)?;
        let end_tile = self.tiled.world_to_tile(end)?;
        let (start_x, start_y) = (start_tile.x as usize, start_tile.y as usize);
        let (end_x, end_y) = (end_tile.x as usize, end_tile.y as usize);

        let result = astar(
            &(start_x, start_y),
            |&(x, y)| self.get_walkable_successors(x, y, collision_grid),
            |&(x, y)| self.manhattan_distance((x, y), (end_x, end_y)),
            |&(x, y)| x == end_x && y == end_y,
        );

        result.map(|(path, _)| {
            path.iter()
                .map(|&(x, y)| self.tiled.tile_to_world(x, y))
                .collect()
        })
    }
}
