use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};

use arrayvec::ArrayVec;
use rand::{distributions::Standard, rngs::StdRng, Rng, SeedableRng};

use crate::game::map::items::helpers::{
    random_book, BACKPACK, GOD_AXE, LEATHER_ARM_GUARD, OBSIDIAN_SHARD, QUIVER, RAGS, STONE_KNIFE,
    STONE_SHOVEL, STONE_SPEAR, WOODEN_ARROW, WOODEN_SHORTBOW,
};
use crate::game::AmmoType;

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
                        random_book(),
                        Item::new(STONE_KNIFE),
                        Item::new(OBSIDIAN_SHARD),
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
        let count: usize = rng.gen_range(5..20);
        let mut blocked_tiles = HashSet::with_capacity(100);
        for _ in 0..count {
            let mut pos = rng.gen_range(0..Chunk::USIZE);
            while blocked_tiles.contains(&pos) {
                pos = rng.gen_range(0..Chunk::USIZE);
            }
            blocked_tiles.insert(pos);
            if pos > 0 {
                blocked_tiles.insert(pos - 1);
            }
            if pos < Chunk::USIZE - 1 {
                blocked_tiles.insert(pos + 1);
            }
            if pos > Chunk::SIZE as usize - 1 {
                blocked_tiles.insert(pos - Chunk::SIZE as usize);
            }
            if pos < Chunk::USIZE - 1 - Chunk::SIZE as usize {
                blocked_tiles.insert(pos + Chunk::SIZE as usize);
            }

            tiles
                .get_mut(pos)
                .unwrap()
                .items
                .push(match rng.gen_range(0..4) {
                    0 => Item::new(WOODEN_SHORTBOW),
                    1 => Item::new(QUIVER).with_items_inside(vec![Item::new(WOODEN_ARROW); 30]),
                    2 => Item::new(GOD_AXE),
                    3 => Item::new(STONE_SPEAR),
                    _ => unreachable!(),
                });
        }
        Chunk { pos, tiles }
    }
}
