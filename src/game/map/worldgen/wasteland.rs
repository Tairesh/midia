use arrayvec::ArrayVec;
use bracket_noise::prelude::FastNoise;
use geometry::Point;
use rand::{distributions::Standard, prelude::StdRng, Rng, SeedableRng};

use super::{
    super::{
        terrains::{DeadTrees, Dirt, Grass, LiveTrees, Tree},
        Terrain, Tile, TilePos,
    },
    chunk_seed, Chunk, ChunkPos, WorldGen,
};

pub struct Wasteland;

impl WorldGen for Wasteland {
    fn generate(world_seed: u64, noise: &FastNoise, pos: ChunkPos) -> Chunk {
        let mut rng = StdRng::seed_from_u64(chunk_seed(world_seed, pos));
        let mut surface = ArrayVec::new();
        for i in 0..Chunk::USIZE {
            let point = Point::from_chunk(pos, i);
            let humidity = noise.get_noise(
                point.x as f32 / Chunk::NOISE_SIZE,
                point.y as f32 / Chunk::NOISE_SIZE,
            );
            let terrain: Terrain = if rng.gen_bool(0.05) {
                Tree::new(if humidity > 0.0 + rng.gen_range(-0.2..=0.2) {
                    rng.sample(LiveTrees)
                } else {
                    rng.sample(DeadTrees)
                })
                .into()
            } else if humidity > 0.0 {
                Grass::new(rng.sample(Standard), humidity < 0.1).into()
            } else {
                Dirt::new(rng.sample(Standard)).into()
            };

            let tile = Tile::new(terrain);
            surface.push(tile);
        }
        Chunk { pos, surface }
    }
}
