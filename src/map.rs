use super::*;

use macroquad_tiled as tiled;

pub struct Map {
    tilemap: macroquad_tiled::Map
}

impl Map {
    pub async fn new(tex_path: &str, data_path: &str) -> Self {
        let tex = load_texture(tex_path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);

        let data = load_string(data_path).await.unwrap();
        let tilemap = tiled::load_map(
            &data, &[("tileset.png", tex)],
            &[]
        ).unwrap();

        Self { tilemap }
    }

    pub fn update(&mut self) {
        // add colliders to prop layer
    }

    pub fn draw(&self) {        
        let layer = &self.tilemap.layers["Base"];
        let layer_width = (layer.width * 32) as f32;
        let layer_height = (layer.height * 32) as f32;

        self.tilemap.draw_tiles("Base", Rect::new(
            0.0, 0.0,
            layer_width, layer_height
        ), None);

        self.tilemap.draw_tiles("Props", Rect::new(
            0.0, 0.0,
            layer_width, layer_height
        ), None);
    }
}