use std::hash::{DefaultHasher, Hash, Hasher};

use bracket_noise::prelude::FastNoise;

use super::{Chunk, ChunkPos};

pub mod wasteland;

pub trait WorldGen {
    fn generate(world_seed: u64, noise: &FastNoise, pos: ChunkPos) -> Chunk;
}

#[derive(Hash)]
struct ChunkUnique {
    pos: ChunkPos,
    world_seed: u64,
}

fn chunk_seed(world_seed: u64, pos: ChunkPos) -> u64 {
    let mut hasher = DefaultHasher::new();
    let seed = ChunkUnique { pos, world_seed };
    seed.hash(&mut hasher);
    hasher.finish()
}
