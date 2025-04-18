use dyrah_shared::{Position, Vec2, map::TiledMap, vec2};
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
                if let Some((world_x, world_y)) = map.tiled.tile_to_world(x, y) {
                    if !map.tiled.is_walkable("colliders", world_x, world_y) {
                        self.grid[y * self.width + x] = true;
                    }
                }
            }
        }

        world.query(|_, _: &Collider, pos: &Position| {
            if let Some((tile_x, tile_y)) =
                map.tiled.world_to_tile(pos.vec.x as u32, pos.vec.y as u32)
            {
                if tile_x < self.width && tile_y < self.height {
                    self.grid[tile_y * self.width + tile_x] = true;
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

    pub fn get_spawn(&self, name: &str) -> Option<Vec2> {
        self.tiled
            .get_object("spawns", name)
            .map(|o| vec2(o.x, o.y))
    }

    pub fn is_walkable(&self, pos: Vec2, grid: &CollisionGrid) -> bool {
        if let Some((tile_x, tile_y)) = self.tiled.world_to_tile(pos.x as u32, pos.y as u32) {
            grid.is_walkable(tile_x, tile_y)
        } else {
            false
        }
    }

    pub fn get_tile(&self, vec: Vec2, grid: &CollisionGrid) -> Option<Vec2> {
        if !self.is_walkable(vec, grid) {
            return None;
        }

        self.tiled
            .get_tile("floor", vec.x as u32, vec.y as u32)
            .and_then(|(tile_x, tile_y)| self.tiled.tile_to_world(tile_x, tile_y))
            .map(|(x, y)| vec2(x as f32, y as f32))
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

    pub fn find_path(&self, start: Vec2, end: Vec2, grid: &CollisionGrid) -> Option<Vec<Vec2>> {
        let (start_x, start_y) = self.tiled.world_to_tile(start.x as u32, start.y as u32)?;
        let (end_x, end_y) = self.tiled.world_to_tile(end.x as u32, end.y as u32)?;

        let result = astar(
            &(start_x, start_y),
            |&(x, y)| self.get_walkable_successors(x, y, grid),
            |&(x, y)| self.manhattan_distance((x, y), (end_x, end_y)),
            |&(x, y)| x == end_x && y == end_y,
        );

        result.map(|(path, _)| {
            path.iter()
                .map(|&(x, y)| {
                    let (x, y) = self.tiled.tile_to_world(x, y).unwrap();
                    vec2(x as f32, y as f32)
                })
                .collect()
        })
    }
}
