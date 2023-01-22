use geometry::Vec2;
use tetra::{
    graphics::{DrawParams, Rectangle, Texture},
    Context,
};

static REGIONS: phf::Map<&'static str, Rectangle> = phf::phf_map! {
    "gazan" => Rectangle::new(0.0, 0.0, 10.0, 10.0),
    "nyarnik" => Rectangle::new(10.0, 0.0, 10.0, 10.0),
    "totik" => Rectangle::new(20.0, 0.0, 10.0, 10.0),
    "lagnam" => Rectangle::new(30.0, 0.0, 10.0, 10.0),
    "bug" => Rectangle::new(40.0, 0.0, 10.0, 10.0),
    "highlight" => Rectangle::new(90.0, 0.0, 10.0, 10.0),
    "dirt1" => Rectangle::new(0.0, 10.0, 10.0, 10.0),
    "dirt2" => Rectangle::new(10.0, 10.0, 10.0, 10.0),
    "dirt3" => Rectangle::new(20.0, 10.0, 10.0, 10.0),
    "dirt4" => Rectangle::new(30.0, 10.0, 10.0, 10.0),
    "dirt5" => Rectangle::new(40.0, 10.0, 10.0, 10.0),
    "boulder_huge" => Rectangle::new(50.0, 10.0, 10.0, 10.0),
    "boulder_middle" => Rectangle::new(60.0, 10.0, 10.0, 10.0),
    "boulder_small" => Rectangle::new(70.0, 10.0, 10.0, 10.0),
    "pit" => Rectangle::new(80.0, 10.0, 10.0, 10.0),
    "stone_shards" => Rectangle::new(90.0, 10.0, 10.0, 10.0),
    "wooden_splinter" => Rectangle::new(90.0, 20.0, 10.0, 10.0),
    "grass1" => Rectangle::new(0.0, 20.0, 10.0, 10.0),
    "grass2" => Rectangle::new(10.0, 20.0, 10.0, 10.0),
    "grass3" => Rectangle::new(20.0, 20.0, 10.0, 10.0),
    "grass4" => Rectangle::new(30.0, 20.0, 10.0, 10.0),
    "grass5" => Rectangle::new(40.0, 20.0, 10.0, 10.0),
    "grass6" => Rectangle::new(50.0, 20.0, 10.0, 10.0),
    "grass7" => Rectangle::new(60.0, 20.0, 10.0, 10.0),
    "grass8" => Rectangle::new(0.0, 40.0, 10.0, 10.0),
    "grass9" => Rectangle::new(10.0, 40.0, 10.0, 10.0),
    "grass10" => Rectangle::new(20.0, 40.0, 10.0, 10.0),
    "grass11" => Rectangle::new(30.0, 40.0, 10.0, 10.0),
    "grass12" => Rectangle::new(40.0, 40.0, 10.0, 10.0),
    "grass13" => Rectangle::new(50.0, 40.0, 10.0, 10.0),
    "grass14" => Rectangle::new(60.0, 40.0, 10.0, 10.0),
    "dead_grass1" => Rectangle::new(0.0, 30.0, 10.0, 10.0),
    "dead_grass2" => Rectangle::new(10.0, 30.0, 10.0, 10.0),
    "dead_grass3" => Rectangle::new(20.0, 30.0, 10.0, 10.0),
    "dead_grass4" => Rectangle::new(30.0, 30.0, 10.0, 10.0),
    "dead_grass5" => Rectangle::new(40.0, 30.0, 10.0, 10.0),
    "dead_grass6" => Rectangle::new(50.0, 30.0, 10.0, 10.0),
    "dead_grass7" => Rectangle::new(60.0, 30.0, 10.0, 10.0),
    "dead_grass8" => Rectangle::new(0.0, 50.0, 10.0, 10.0),
    "dead_grass9" => Rectangle::new(10.0, 50.0, 10.0, 10.0),
    "dead_grass10" => Rectangle::new(20.0, 50.0, 10.0, 10.0),
    "dead_grass11" => Rectangle::new(30.0, 50.0, 10.0, 10.0),
    "dead_grass12" => Rectangle::new(40.0, 50.0, 10.0, 10.0),
    "dead_grass13" => Rectangle::new(50.0, 50.0, 10.0, 10.0),
    "dead_grass14" => Rectangle::new(60.0, 50.0, 10.0, 10.0),
    "book" => Rectangle::new(0.0, 60.0, 10.0, 10.0),
    "chest" => Rectangle::new(10.0, 60.0, 10.0, 10.0),
    "opened_chest" => Rectangle::new(20.0, 60.0, 10.0, 10.0),
    "corpse" => Rectangle::new(30.0, 60.0, 10.0, 10.0),
    "flesh" => Rectangle::new(40.0, 60.0, 10.0, 10.0),
    "leather_oversleeve" => Rectangle::new(50.0, 60.0, 10.0, 10.0),
    "shovel" => Rectangle::new(0.0, 70.0, 10.0, 10.0),
    "knife" => Rectangle::new(10.0, 70.0, 10.0, 10.0),
    "axe" => Rectangle::new(20.0, 70.0, 10.0, 10.0),
    "cloak" => Rectangle::new(30.0, 70.0, 10.0, 10.0),
    "hat" => Rectangle::new(40.0, 70.0, 10.0, 10.0),
    "rags" => Rectangle::new(50.0, 70.0, 10.0, 10.0),
    "backpack" => Rectangle::new(60.0, 70.0, 10.0, 10.0),
    "wooden_sap" => Rectangle::new(0.0, 80.0, 10.0, 10.0),
    "stone_sap" => Rectangle::new(10.0, 80.0, 10.0, 10.0),
    "metal_sap" => Rectangle::new(20.0, 80.0, 10.0, 10.0),
    "demonic_sap" => Rectangle::new(30.0, 80.0, 10.0, 10.0),
    "mt" => Rectangle::new(0.0, 90.0, 10.0, 10.0),
    "lt" => Rectangle::new(10.0, 90.0, 10.0, 10.0),
    "minus" => Rectangle::new(20.0, 90.0, 10.0, 10.0),
    "plus" => Rectangle::new(30.0, 90.0, 10.0, 10.0),
    "dead_tree" => Rectangle::new(70.0, 20.0, 20.0, 20.0),
    "dead_pine" => Rectangle::new(70.0, 40.0, 20.0, 20.0),
    "dead_hickory" => Rectangle::new(70.0, 60.0, 20.0, 20.0),
};

#[derive(Debug)]
pub struct Tileset {
    pub tile_size: i32,
    pub texture: Texture,
}

impl Tileset {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            tile_size: 10, // TODO: support custom tilesets
            texture: Texture::from_encoded(ctx, include_bytes!("../../assets/img/tileset.png"))?,
        })
    }

    pub fn get_size(name: &str) -> Vec2 {
        REGIONS
            .get(name)
            .map_or(Vec2::new(10.0, 10.0), |r| Vec2::new(r.width, r.height))
    }

    pub fn draw_region<P: Into<DrawParams>>(&self, ctx: &mut Context, name: &str, params: P) {
        let region = REGIONS
            .get(name)
            .copied()
            .unwrap_or_else(|| Rectangle::new(80.0, 0.0, 10.0, 10.0));
        self.texture.draw_region(ctx, region, params);
    }
}
