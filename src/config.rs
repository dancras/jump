use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ArenaConfig {
    pub rows: u16,
    pub cols: u16,
    pub tile_size: f32,
    pub tiles: Vec<u8>
}

impl ArenaConfig {
    pub fn arena_width(&self) -> f32 {
        self.cols as f32 * self.tile_size
    }

    pub fn tile(&self, x: u16, y: u16) -> u8 {
        let i = y * self.cols + x;
        self.tiles[i as usize]
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JumpConfig {
    pub arena: ArenaConfig
}
