use geometry::Vec2;
use tetra::{
    graphics::{DrawParams, Rectangle, Texture},
    Context,
};

const REGIONS: phf::Map<&'static str, Rectangle> = phf::phf_map! {
    "empty" => Rectangle::new(0.0, 0.0, 10.0, 10.0),
    "highlight" => Rectangle::new(10.0, 0.0, 10.0, 10.0),
    "mt" => Rectangle::new(20.0, 0.0, 10.0, 10.0),
    "lt" => Rectangle::new(30.0, 0.0, 10.0, 10.0),
    "minus" => Rectangle::new(40.0, 0.0, 10.0, 10.0),
    "plus" => Rectangle::new(50.0, 0.0, 10.0, 10.0),

    "dirt1" => Rectangle::new(0.0, 10.0, 10.0, 10.0),
    "dirt2" => Rectangle::new(10.0, 10.0, 10.0, 10.0),
    "dirt3" => Rectangle::new(20.0, 10.0, 10.0, 10.0),
    "dirt4" => Rectangle::new(30.0, 10.0, 10.0, 10.0),
    "dirt5" => Rectangle::new(40.0, 10.0, 10.0, 10.0),
    "dirt6" => Rectangle::new(50.0, 10.0, 10.0, 10.0),
    "dirt7" => Rectangle::new(60.0, 10.0, 10.0, 10.0),
    "dirt8" => Rectangle::new(70.0, 10.0, 10.0, 10.0),
    "dirt9" => Rectangle::new(80.0, 10.0, 10.0, 10.0),
    "dirt10" => Rectangle::new(90.0, 10.0, 10.0, 10.0),

    "grass1" => Rectangle::new(0.0, 20.0, 10.0, 10.0),
    "grass2" => Rectangle::new(10.0, 20.0, 10.0, 10.0),
    "grass3" => Rectangle::new(20.0, 20.0, 10.0, 10.0),
    "grass4" => Rectangle::new(30.0, 20.0, 10.0, 10.0),
    "grass5" => Rectangle::new(40.0, 20.0, 10.0, 10.0),
    "grass6" => Rectangle::new(50.0, 20.0, 10.0, 10.0),
    "grass7" => Rectangle::new(60.0, 20.0, 10.0, 10.0),
    "grass8" => Rectangle::new(70.0, 20.0, 10.0, 10.0),
    "grass9" => Rectangle::new(80.0, 20.0, 10.0, 10.0),
    "grass10" => Rectangle::new(90.0, 20.0, 10.0, 10.0),

    "grass11" => Rectangle::new(0.0, 30.0, 10.0, 10.0),
    "grass12" => Rectangle::new(10.0, 30.0, 10.0, 10.0),
    "grass13" => Rectangle::new(20.0, 30.0, 10.0, 10.0),
    "grass14" => Rectangle::new(30.0, 30.0, 10.0, 10.0),
    "grass15" => Rectangle::new(40.0, 30.0, 10.0, 10.0),
    "grass16" => Rectangle::new(50.0, 30.0, 10.0, 10.0),
    "grass17" => Rectangle::new(60.0, 30.0, 10.0, 10.0),
    "grass18" => Rectangle::new(70.0, 30.0, 10.0, 10.0),
    "grass19" => Rectangle::new(80.0, 30.0, 10.0, 10.0),
    "grass20" => Rectangle::new(90.0, 30.0, 10.0, 10.0),

    "chest" => Rectangle::new(0.0, 40.0, 10.0, 10.0),
    "chest_open" => Rectangle::new(10.0, 40.0, 10.0, 10.0),
    "boulder_huge" => Rectangle::new(20.0, 40.0, 10.0, 10.0),
    "boulder" => Rectangle::new(30.0, 40.0, 10.0, 10.0),
    "boulder_small" => Rectangle::new(40.0, 40.0, 10.0, 10.0),
    "pit" => Rectangle::new(50.0, 40.0, 10.0, 10.0),

    "tree_dead" => Rectangle::new(0.0, 50.0, 20.0, 20.0),
    "pine_dead" => Rectangle::new(20.0, 50.0, 20.0, 20.0),
    "hickory_dead" => Rectangle::new(40.0, 50.0, 20.0, 20.0),
    "willow_dead" => Rectangle::new(60.0, 50.0, 20.0, 20.0),
    "birch_dead" => Rectangle::new(80.0, 50.0, 20.0, 20.0),

    "tree" => Rectangle::new(0.0, 70.0, 20.0, 20.0),
    "pine" => Rectangle::new(20.0, 70.0, 20.0, 20.0),
    "hickory" => Rectangle::new(40.0, 70.0, 20.0, 20.0),
    "willow" => Rectangle::new(60.0, 70.0, 20.0, 20.0),
    "birch" => Rectangle::new(80.0, 70.0, 20.0, 20.0),

    "gazan" => Rectangle::new(0.0, 90.0, 10.0, 10.0),
    "nyarnik" => Rectangle::new(10.0, 90.0, 10.0, 10.0),
    "totik" => Rectangle::new(20.0, 90.0, 10.0, 10.0),
    "lagnam" => Rectangle::new(30.0, 90.0, 10.0, 10.0),
    "giant_bug" => Rectangle::new(40.0, 90.0, 10.0, 10.0),

    "cloak" => Rectangle::new(0.0, 100.0, 10.0, 10.0),
    "hat" => Rectangle::new(10.0, 100.0, 10.0, 10.0),
    "rags" => Rectangle::new(20.0, 100.0, 10.0, 10.0),
    "backpack" => Rectangle::new(30.0, 100.0, 10.0, 10.0),
    "arm_guard" => Rectangle::new(40.0, 100.0, 10.0, 10.0),
    "glove_right" => Rectangle::new(50.0, 100.0, 10.0, 10.0),
    "glove_left" => Rectangle::new(60.0, 100.0, 10.0, 10.0),

    "knife_wood" => Rectangle::new(0.0, 120.0, 10.0, 10.0),
    "knife_stone" => Rectangle::new(10.0, 120.0, 10.0, 10.0),
    "knife_metal" => Rectangle::new(20.0, 120.0, 10.0, 10.0),
    "knife_demonic" => Rectangle::new(30.0, 120.0, 10.0, 10.0),
    "axe_wood" => Rectangle::new(40.0, 120.0, 10.0, 10.0),
    "axe_stone" => Rectangle::new(50.0, 120.0, 10.0, 10.0),
    "axe_metal" => Rectangle::new(60.0, 120.0, 10.0, 10.0),
    "axe_demonic" => Rectangle::new(70.0, 120.0, 10.0, 10.0),
    "buckler" => Rectangle::new(80.0, 120.0, 10.0, 10.0),
    "knife_bone" => Rectangle::new(90.0, 120.0, 10.0, 10.0),

    "sap_wood" => Rectangle::new(0.0, 130.0, 10.0, 10.0),
    "sap_stone" => Rectangle::new(10.0, 130.0, 10.0, 10.0),
    "sap_metal" => Rectangle::new(20.0, 130.0, 10.0, 10.0),
    "sap_demonic" => Rectangle::new(30.0, 130.0, 10.0, 10.0),
    "sword_wood" => Rectangle::new(40.0, 130.0, 10.0, 10.0),
    "sword_stone" => Rectangle::new(50.0, 130.0, 10.0, 10.0),
    "sword_metal" => Rectangle::new(60.0, 130.0, 10.0, 10.0),
    "sword_demonic" => Rectangle::new(70.0, 130.0, 10.0, 10.0),
    "sword_obsidian" => Rectangle::new(80.0, 130.0, 10.0, 10.0),
    "knife_obsidian" => Rectangle::new(90.0, 130.0, 10.0, 10.0),

    "club_wood" => Rectangle::new(0.0, 140.0, 10.0, 10.0),
    "club_stone" => Rectangle::new(10.0, 140.0, 10.0, 10.0),
    "club_metal" => Rectangle::new(20.0, 140.0, 10.0, 10.0),
    "club_demonic" => Rectangle::new(30.0, 140.0, 10.0, 10.0),
    "mace_wood" => Rectangle::new(40.0, 140.0, 10.0, 10.0),
    "mace_stone" => Rectangle::new(50.0, 140.0, 10.0, 10.0),
    "mace_metal" => Rectangle::new(60.0, 140.0, 10.0, 10.0),
    "mace_demonic" => Rectangle::new(70.0, 140.0, 10.0, 10.0),
    "knuckle_obsidian" => Rectangle::new(80.0, 140.0, 10.0, 10.0),
    "knuckle_bone" => Rectangle::new(90.0, 140.0, 10.0, 10.0),

    "knuckle_wood" => Rectangle::new(0.0, 150.0, 10.0, 10.0),
    "knuckle_stone" => Rectangle::new(10.0, 150.0, 10.0, 10.0),
    "knuckle_metal" => Rectangle::new(20.0, 150.0, 10.0, 10.0),
    "knuckle_demonic" => Rectangle::new(30.0, 150.0, 10.0, 10.0),
    "hammer_wood" => Rectangle::new(40.0, 150.0, 10.0, 10.0),
    "hammer_stone" => Rectangle::new(50.0, 150.0, 10.0, 10.0),
    "hammer_metal" => Rectangle::new(60.0, 150.0, 10.0, 10.0),
    "hammer_demonic" => Rectangle::new(70.0, 150.0, 10.0, 10.0),

    "spear_wood" => Rectangle::new(0.0, 160.0, 10.0, 10.0),
    "spear_stone" => Rectangle::new(10.0, 160.0, 10.0, 10.0),
    "spear_metal" => Rectangle::new(20.0, 160.0, 10.0, 10.0),
    "spear_demonic" => Rectangle::new(30.0, 160.0, 10.0, 10.0),
    "pike_wood" => Rectangle::new(40.0, 160.0, 10.0, 10.0),
    "pike_stone" => Rectangle::new(50.0, 160.0, 10.0, 10.0),
    "pike_metal" => Rectangle::new(60.0, 160.0, 10.0, 10.0),
    "pike_demonic" => Rectangle::new(70.0, 160.0, 10.0, 10.0),

    "bow_wood" => Rectangle::new(0.0, 170.0, 10.0, 10.0),
    "bow_demonic" => Rectangle::new(10.0, 170.0, 10.0, 10.0),
    "crossbow_wood" => Rectangle::new(20.0, 170.0, 10.0, 10.0),
    "crossbow_metal" => Rectangle::new(30.0, 170.0, 10.0, 10.0),
    "crossbow_demonic" => Rectangle::new(40.0, 170.0, 10.0, 10.0),
    "sling" => Rectangle::new(50.0, 170.0, 10.0, 10.0),
    "pipe" => Rectangle::new(60.0, 170.0, 10.0, 10.0),
    "pistol" => Rectangle::new(70.0, 170.0, 10.0, 10.0),
    "rifle" => Rectangle::new(80.0, 170.0, 10.0, 10.0),
    "slingshot" => Rectangle::new(90.0, 170.0, 10.0, 10.0),

    "arrow_wood" => Rectangle::new(0.0, 180.0, 10.0, 10.0),
    "arrow_stone" => Rectangle::new(10.0, 180.0, 10.0, 10.0),
    "arrow_metal" => Rectangle::new(20.0, 180.0, 10.0, 10.0),
    "arrow_demonic" => Rectangle::new(30.0, 180.0, 10.0, 10.0),
    "arrow_obsidian" => Rectangle::new(40.0, 180.0, 10.0, 10.0),
    "arrow_explosive" => Rectangle::new(50.0, 180.0, 10.0, 10.0),
    "arrow_bone" => Rectangle::new(60.0, 180.0, 10.0, 10.0),
    "boomgranate_fruit" => Rectangle::new(70.0, 180.0, 10.0, 10.0),

    "rock" => Rectangle::new(0.0, 190.0, 10.0, 10.0),
    "sharp_rock" => Rectangle::new(10.0, 190.0, 10.0, 10.0),
    "pebble" => Rectangle::new(20.0, 190.0, 10.0, 10.0),
    "metal_chunk" => Rectangle::new(30.0, 190.0, 10.0, 10.0),
    "sharp_metal_chunk" => Rectangle::new(40.0, 190.0, 10.0, 10.0),
    "metal_shot" => Rectangle::new(50.0, 190.0, 10.0, 10.0),
    "demonic_chunk" => Rectangle::new(60.0, 190.0, 10.0, 10.0),
    "sharp_demonic_chunk" => Rectangle::new(70.0, 190.0, 10.0, 10.0),
    "demonic_shot" => Rectangle::new(80.0, 190.0, 10.0, 10.0),
    "obsidian_chunk" => Rectangle::new(90.0, 190.0, 10.0, 10.0),

    "corpse" => Rectangle::new(0.0, 200.0, 10.0, 10.0),
    "book" => Rectangle::new(10.0, 200.0, 10.0, 10.0),
    "wooden_splinter" => Rectangle::new(20.0, 200.0, 10.0, 10.0),
    "flesh_chunk" => Rectangle::new(30.0, 200.0, 10.0, 10.0),
    "bone" => Rectangle::new(40.0, 200.0, 10.0, 10.0),
    "lazuli" => Rectangle::new(50.0, 200.0, 10.0, 10.0),

    "shovel_wood" => Rectangle::new(0.0, 210.0, 10.0, 10.0),
    "shovel_stone" => Rectangle::new(10.0, 210.0, 10.0, 10.0),
    "shovel_metal" => Rectangle::new(20.0, 210.0, 10.0, 10.0),
    "shovel_demonic" => Rectangle::new(30.0, 210.0, 10.0, 10.0),
};
const TILE_SIZE: i32 = 10;

#[derive(Debug)]
pub struct Tileset {
    pub tile_size: i32,
    pub texture: Texture,
}

impl Tileset {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            tile_size: TILE_SIZE, // TODO: support custom tilesets
            texture: Texture::from_encoded(ctx, include_bytes!("../../assets/img/tileset.png"))?,
        })
    }

    pub fn get_size(name: &str) -> Vec2 {
        REGIONS
            .get(name)
            .map_or(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32), |r| {
                Vec2::new(r.width, r.height)
            })
    }

    pub fn draw_region<P: Into<DrawParams>>(&self, ctx: &mut Context, name: &str, params: P) {
        let region = REGIONS
            .get(name)
            .copied()
            .unwrap_or_else(|| Rectangle::new(0.0, 0.0, 10.0, 10.0));
        self.texture.draw_region(ctx, region, params);
    }
}
