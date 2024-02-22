use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use arrayvec::ArrayVec;
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

use crate::game::map::items::helpers::{
    DEMONIC_PIKE, GOD_AXE, OBSIDIAN_BOLT, OBSIDIAN_SHARD, QUIVER, STONE_KNIFE, WOODEN_ARROW,
    WOODEN_CROSSBOW, WOODEN_SHORTBOW,
};

use super::{
    terrains::{Boulder, Chest, Dirt, Grass, Tree},
    ChunkPos, Item, Tile,
};

#[derive(Hash)]
struct ChunkUnique {
    pos: ChunkPos,
    world_seed: String,
}

fn chunk_seed(world_seed: String, pos: ChunkPos) -> u64 {
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

    pub fn generate(world_seed: String, pos: ChunkPos) -> Self {
        let mut rng = StdRng::seed_from_u64(chunk_seed(world_seed, pos));
        let mut tiles = ArrayVec::new();
        for _ in 0..Chunk::USIZE {
            tiles.push(Tile::new(if rng.gen_bool(0.005) {
                Tree::new(rng.sample(Standard)).into()
            } else if rng.gen_bool(0.003) {
                Chest::new(
                    vec![
                        Item::new(STONE_KNIFE),
                        Item::new(OBSIDIAN_SHARD),
                        Item::new(GOD_AXE),
                        Item::new(DEMONIC_PIKE),
                        Item::new(QUIVER)
                            .with_items_inside(vec![Item::new(OBSIDIAN_BOLT); 3])
                            .with_items_inside(vec![Item::new(WOODEN_ARROW); 3]),
                        Item::new(WOODEN_CROSSBOW),
                        Item::new(WOODEN_SHORTBOW),
                    ],
                    false,
                )
                .into()
            } else if rng.gen_bool(0.01) {
                Boulder::new(rng.sample(Standard)).into()
            } else if rng.gen_bool(0.5) {
                Grass::new(rng.sample(Standard), rng.gen_bool(0.1)).into()
            } else {
                Dirt::new(rng.sample(Standard)).into()
            }));
        }
        Chunk { pos, tiles }
    }
}
