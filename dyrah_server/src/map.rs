use dyrah_shared::map::TiledMap;
use secs::World;

use crate::components::{Collider, TilePos};
use glam::{IVec2, Vec2};

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
                let tile_pos = IVec2::new(x as i32, y as i32);
                if !map.tiled.is_walkable("colliders", tile_pos) {
                    self.grid[y * self.width + x] = true;
                }
            }
        }

        world.query(|_, _: &Collider, tile_pos: &TilePos| {
            let x = tile_pos.vec.x as usize;
            let y = tile_pos.vec.y as usize;

            if x < self.width && y < self.height {
                self.grid[y * self.width + x] = true;
            }
        });
    }

    pub fn is_walkable(&self, tile_pos: IVec2) -> bool {
        let (x, y) = (tile_pos.x as usize, tile_pos.y as usize);
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

    pub fn get_spawn(&self, name: &str) -> Option<IVec2> {
        self.tiled
            .get_object("spawns", name)
            .map(|o| self.tiled.world_to_tile(Vec2::new(o.x, o.y)))
    }

    pub fn is_walkable(&self, tile_pos: IVec2, grid: &CollisionGrid) -> bool {
        grid.is_walkable(tile_pos)
    }
}
