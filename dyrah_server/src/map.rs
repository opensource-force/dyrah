use dyrah_shared::{TILE_OFFSET, Vec2, map::TiledMap, vec2};
use pathfinding::prelude::astar;

pub struct Map {
    pub tiled: TiledMap,
    path_grid: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(path: &str, layer_name: &str) -> Self {
        let mut map = Self {
            tiled: TiledMap::new(path),
            path_grid: Vec::new(),
        };

        map.path_grid = map.create_pathfinding_grid(layer_name);
        map
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

    pub fn find_path(&self, start: Vec2, end: Vec2) -> Option<Vec<Vec2>> {
        let start = self.tiled.world_to_tile(start)?;
        let end = self.tiled.world_to_tile(end)?;
        let (start_x, start_y) = (start.x as usize, start.y as usize);
        let (end_x, end_y) = (end.x as usize, end.y as usize);

        if start_x >= self.path_grid.len()
            || start_y >= self.path_grid[0].len()
            || end_x >= self.path_grid.len()
            || end_y >= self.path_grid[0].len()
        {
            return None;
        }

        let result = astar(
            &(start_x, start_y),
            |&(x, y)| self.get_walkable_successors(x, y),
            |&(x, y)| self.manhattan_distance((x, y), (end_x, end_y)),
            |&(x, y)| x == end_x && y == end_y,
        );

        result.map(|(path, _)| path.iter().map(|p| self.tiled.tile_to_world(*p)).collect())
    }

    fn create_pathfinding_grid(&self, collision_layer: &str) -> Vec<Vec<bool>> {
        let mut grid = vec![vec![true; self.tiled.height as usize]; self.tiled.width as usize];

        if let Some(layer) = self.tiled.get_layer(collision_layer) {
            for y in 0..self.tiled.height {
                for x in 0..self.tiled.width {
                    let index = (y * layer.width.unwrap() + x) as usize;
                    if let Some(data) = &layer.data {
                        if data.get(index).map_or(false, |&tile| tile != 0) {
                            grid[x as usize][y as usize] = false;
                        }
                    }
                }
            }
        }
        grid
    }

    fn get_walkable_successors(&self, x: usize, y: usize) -> Vec<((usize, usize), u32)> {
        let mut successors = Vec::new();
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0
                && ny >= 0
                && nx < self.path_grid.len() as i32
                && ny < self.path_grid[0].len() as i32
                && self.path_grid[nx as usize][ny as usize]
            {
                successors.push(((nx as usize, ny as usize), 1));
            }
        }
        successors
    }

    fn manhattan_distance(&self, a: (usize, usize), b: (usize, usize)) -> u32 {
        ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
    }
}
