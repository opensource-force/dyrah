use glam::IVec2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TilePos {
    pub vec: IVec2,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TargetTilePos {
    pub vec: IVec2,
}
