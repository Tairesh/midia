use std::collections::{HashMap, HashSet};

use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use geometry::Point;

pub use chunk::Chunk;
pub use fov::{field_of_view_set, Fov, FovMap};
pub use items::Item;
pub use passage::Passage;
pub use pos::{ChunkPos, TilePos};
pub use terrain::{Terrain, TerrainInteract, TerrainView};
pub use tile::Tile;

mod chunk;
mod fov;
pub mod items;
mod passage;
mod pos;
mod terrain;
pub mod terrains;
mod tile;

pub struct Map {
    pub seed: u64,
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub changed: HashSet<ChunkPos>,
    noise: FastNoise,
}

impl Map {
    pub fn new(seed: u64, chunks: HashMap<ChunkPos, Chunk>, changed: HashSet<ChunkPos>) -> Self {
        let mut noise = FastNoise::seeded(seed);
        noise.set_noise_type(NoiseType::PerlinFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(5);
        noise.set_fractal_gain(0.6);
        noise.set_fractal_lacunarity(2.0);
        noise.set_frequency(2.0);

        Self {
            seed,
            chunks,
            changed,
            noise,
        }
    }

    pub fn get_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(self.seed, &self.noise, *pos))
    }

    pub fn get_chunk_mut(&mut self, pos: ChunkPos) -> &mut Chunk {
        self.changed.insert(pos);
        self.chunks
            .entry(pos)
            .or_insert_with_key(|pos| Chunk::generate(self.seed, &self.noise, *pos))
    }

    pub fn get_tile_opt(&self, pos: Point) -> Option<&Tile> {
        let (chunk, pos) = pos.to_chunk();
        self.chunks.get(&chunk).map(|c| &c.surface[pos])
    }

    pub fn get_tile(&mut self, pos: Point) -> &Tile {
        let (chunk, pos) = pos.to_chunk();
        let chunk = self.get_chunk(chunk);
        &chunk.surface[pos]
    }

    pub fn get_tile_mut(&mut self, pos: Point) -> &mut Tile {
        let (chunk, pos) = pos.to_chunk();
        let chunk = self.get_chunk_mut(chunk);
        &mut chunk.surface[pos]
    }

    pub fn load_tiles_between(&mut self, left_top: Point, right_bottom: Point) {
        let (ChunkPos { x: lt_x, y: lt_y }, _) = left_top.to_chunk();
        let (ChunkPos { x: rb_x, y: rb_y }, _) = right_bottom.to_chunk();

        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let pos = ChunkPos::new(x, y);
                self.get_chunk(pos);
            }
        }
    }

    pub fn tiles_between(&self, left_top: Point, right_bottom: Point) -> Vec<(Point, &Tile)> {
        let (ChunkPos { x: lt_x, y: lt_y }, _) = left_top.to_chunk();
        let (ChunkPos { x: rb_x, y: rb_y }, _) = right_bottom.to_chunk();

        let mut tiles =
            Vec::with_capacity(((rb_x - lt_x + 1) * (rb_y - lt_y + 1)) as usize * Chunk::USIZE);
        for x in lt_x..=rb_x {
            for y in lt_y..=rb_y {
                let chunk_pos = ChunkPos::new(x, y);
                let chunk = self.chunks.get(&chunk_pos).unwrap();
                for (i, tile) in chunk.surface.iter().enumerate() {
                    tiles.push((TilePos::from_chunk(chunk_pos, i), tile));
                }
            }
        }
        tiles
    }
}

impl FovMap for Map {
    fn is_transparent(&self, pos: Point) -> bool {
        let (chunk, pos) = pos.to_chunk();
        self.chunks
            .get(&chunk)
            .map_or(true, |c| c.surface[pos].terrain.is_transparent())
    }
}
