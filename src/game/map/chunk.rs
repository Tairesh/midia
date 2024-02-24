use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use arrayvec::ArrayVec;
use bracket_noise::prelude::FastNoise;
use geometry::Point;
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

use super::{
    terrains::{Dirt, Grass},
    ChunkPos, Tile, TilePos,
};

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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub tiles: ArrayVec<Tile, { Chunk::USIZE }>,
}

impl Chunk {
    pub const SIZE: i32 = 32;
    pub const USIZE: usize = (Chunk::SIZE * Chunk::SIZE) as usize;
    pub const NOISE_SIZE: f32 = 64.0; // SIZE * frequency

    pub fn generate(world_seed: u64, noise: &FastNoise, pos: ChunkPos) -> Self {
        let mut rng = StdRng::seed_from_u64(chunk_seed(world_seed, pos));
        let mut tiles = ArrayVec::new();
        for i in 0..Chunk::USIZE {
            let point = Point::from_chunk(pos, i);
            let n = noise.get_noise(
                point.x as f32 / Self::NOISE_SIZE,
                point.y as f32 / Self::NOISE_SIZE,
            );
            let terrain = if n > 0.0 {
                Grass::new(rng.sample(Standard), n < 0.1).into()
            } else {
                Dirt::new(rng.sample(Standard)).into()
            };
            let tile = Tile::new(terrain);
            tiles.push(tile);
        }
        Chunk { pos, tiles }
    }
}
