use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub rows: u8,
    pub cols: u8,
    pub tile_size: f32,
    pub tiles: Vec<u8>
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JumpConfig {
    pub arena: ArenaConfig
}
