use std::hash::{Hash, Hasher};

use arrayvec::ArrayVec;

use super::{ChunkPos, Tile};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub surface: ArrayVec<Tile, { Chunk::USIZE }>,
}

impl Chunk {
    pub const SIZE: i32 = 32;
    pub const USIZE: usize = (Chunk::SIZE * Chunk::SIZE) as usize;
    pub const NOISE_SIZE: f32 = 64.0; // SIZE * frequency
}
