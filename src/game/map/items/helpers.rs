#![allow(dead_code)]

use std::collections::HashSet;

use crate::assets::Sprite;
use crate::colors::Colors;
use crate::game::units::Appearance;
use crate::game::{Avatar, ItemPrototype, ItemSize, Material};

use super::Item;

pub const CLOAK: &str = "cloak";
pub const HAT: &str = "hat";
pub const RAGS: &str = "rags";
pub const BACKPACK: &str = "backpack";
pub const QUIVER: &str = "quiver";
pub const LEATHER_ARM_GUARD: &str = "arm_guard_leather";
// pub const LEATHER_LEG_GUARD: &str = "leg_guard_leather";
// pub const LEATHER_LEFT_GLOVE: &str = "glove_left_leather";
// pub const LEATHER_RIGHT_GLOVE: &str = "glove_right_leather";

// pub const BUCKLER: &str = "buckler";

pub const WOODEN_KNIFE: &str = "knife_wood";
pub const STONE_KNIFE: &str = "knife_stone";
pub const IRON_KNIFE: &str = "knife_iron";
pub const STEEL_KNIFE: &str = "knife_steel";
pub const DEMONIC_KNIFE: &str = "knife_demonic";
pub const BONE_KNIFE: &str = "knife_bone";
pub const OBSIDIAN_KNIFE: &str = "knife_obsidian";

// pub const WOODEN_AXE: &str = "axe_wood";
// pub const STONE_AXE: &str = "axe_stone";
// pub const IRON_AXE: &str = "axe_iron";
// pub const STEEL_AXE: &str = "axe_steel";
// pub const DEMONIC_AXE: &str = "axe_demonic";
pub const GOD_AXE: &str = "axe_god";

pub const WOODEN_SAP: &str = "sap_wood";
pub const STONE_SAP: &str = "sap_stone";
pub const IRON_SAP: &str = "sap_iron";
pub const STEEL_SAP: &str = "sap_steel";
pub const DEMONIC_SAP: &str = "sap_demonic";

// pub const WOODEN_SWORD: &str = "sword_wood";
// pub const STONE_SWORD: &str = "sword_stone";
// pub const IRON_SWORD: &str = "sword_iron";
// pub const STEEL_SWORD: &str = "sword_steel";
// pub const DEMONIC_SWORD: &str = "sword_demonic";
// pub const OBSIDIAN_SWORD: &str = "sword_obsidian";

// pub const WOODEN_CLUB: &str = "club_wood";
// pub const STONE_CLUB: &str = "club_stone";
// pub const IRON_CLUB: &str = "club_iron";
// pub const STEEL_CLUB: &str = "club_steel";
// pub const DEMONIC_CLUB: &str = "club_demonic";

// pub const WOODEN_MACE: &str = "mace_wood";
// pub const STONE_MACE: &str = "mace_stone";
// pub const IRON_MACE: &str = "mace_iron";
// pub const STEEL_MACE: &str = "mace_steel";
// pub const DEMONIC_MACE: &str = "mace_demonic";

// pub const WOODEN_KNUCKLE: &str = "knuckle_wood";
// pub const STONE_KNUCKLE: &str = "knuckle_stone";
// pub const IRON_KNUCKLE: &str = "knuckle_iron";
// pub const STEEL_KNUCKLE: &str = "knuckle_steel";
// pub const DEMONIC_KNUCKLE: &str = "knuckle_demonic";
// pub const OBSIDIAN_KNUCKLE: &str = "knuckle_obsidian";
// pub const BONE_KNUCKLE: &str = "knuckle_bone";

// pub const WOODEN_HAMMER: &str = "hammer_wood";
// pub const STONE_HAMMER: &str = "hammer_stone";
// pub const IRON_HAMMER: &str = "hammer_iron";
// pub const STEEL_HAMMER: &str = "hammer_steel";
// pub const DEMONIC_HAMMER: &str = "hammer_demonic";

pub const WOODEN_SPEAR: &str = "spear_wood";
pub const STONE_SPEAR: &str = "spear_stone";
pub const IRON_SPEAR: &str = "spear_iron";
pub const STEEL_SPEAR: &str = "spear_steel";
pub const DEMONIC_SPEAR: &str = "spear_demonic";

pub const WOODEN_PIKE: &str = "pike_wood";
pub const STONE_PIKE: &str = "pike_stone";
pub const IRON_PIKE: &str = "pike_iron";
pub const STEEL_PIKE: &str = "pike_steel";
pub const DEMONIC_PIKE: &str = "pike_demonic";

pub const WOODEN_SHORTBOW: &str = "shortbow_wood";
// pub const DEMONIC_BOW: &str = "bow_demonic";
pub const WOODEN_CROSSBOW: &str = "crossbow_wood";
// pub const IRON_CROSSBOW: &str = "crossbow_iron";
// pub const STEEL_CROSSBOW: &str = "crossbow_steel";
// pub const DEMONIC_CROSSBOW: &str = "crossbow_demonic";
// pub const SLING: &str = "sling";
// pub const BOOMSTICK: &str = "boomstick";
// pub const PISTOL: &str = "pistol";
// pub const RIFLE: &str = "rifle";
// pub const SLINGSHOT: &str = "slingshot";

pub const WOODEN_ARROW: &str = "arrow_wood";
pub const STONE_ARROW: &str = "arrow_stone";
pub const IRON_ARROW: &str = "arrow_iron";
pub const STEEL_ARROW: &str = "arrow_steel";
pub const DEMONIC_ARROW: &str = "arrow_demonic";
pub const OBSIDIAN_ARROW: &str = "arrow_obsidian";
// pub const BONE_ARROW: &str = "arrow_bone";
// pub const BOOM_ARROW: &str = "arrow_boom";
pub const WOODEN_BOLT: &str = "bolt_wood";
pub const STONE_BOLT: &str = "bolt_stone";
pub const IRON_BOLT: &str = "bolt_iron";
pub const STEEL_BOLT: &str = "bolt_steel";
pub const DEMONIC_BOLT: &str = "bolt_demonic";
pub const OBSIDIAN_BOLT: &str = "bolt_obsidian";
// pub const BONE_BOLT: &str = "bolt_bone";
// pub const BOOM_BOLT: &str = "bolt_boom";

// pub const BOOMGRANATE_FRUIT: &str = "boomgranate_fruit";

pub const ROCK: &str = "rock";
// pub const SHARP_ROCK: &str = "sharp_rock";
// pub const PEBBLE: &str = "pebble";
// pub const METAL_CHUNK: &str = "metal_chunk";
// pub const SHARP_METAL_CHUNK: &str = "sharp_metal_chunk";
// pub const METAL_SHOT: &str = "metal_shot";
// pub const DEMONIC_CHUNK: &str = "demonic_chunk";
// pub const SHARP_DEMONIC_CHUNK: &str = "sharp_demonic_chunk";
// pub const DEMONIC_SHOT: &str = "demonic_shot";
pub const OBSIDIAN_SHARD: &str = "shard_obsidian";

pub const CORPSE: &str = "corpse";
pub const BOOK: &str = "book";
pub const WOODEN_SPLINTER: &str = "wooden_splinter";
// pub const FLESH_CHUNK: &str = "flesh_chunk";
// pub const BONE: &str = "bone";
// pub const LAZULI: &str = "lazuli";

// pub const WOODEN_SHOVEL: &str = "shovel_wood";
pub const STONE_SHOVEL: &str = "shovel_stone";
// pub const IRON_SHOVEL: &str = "shovel_iron";
// pub const STEEL_SHOVEL: &str = "shovel_steel";
// pub const DEMONIC_SHOVEL: &str = "shovel_demonic";

#[cfg(test)]
pub fn book_debug() -> Item {
    Item::new(BOOK)
        .with_colored(Colors::BLUE_VIOLET)
        .with_named("strange book")
        .with_readable("Lore Of The Midia")
}

pub fn dead_body(appearance: &Appearance) -> Item {
    let body = Item::custom(ItemPrototype {
        id: CORPSE.to_string(),
        name: format!("dead {}", appearance.body_name()),
        looks_like: Sprite::Corpse,
        size: ItemSize::Huge,
        materials: HashSet::from([Material::Flesh]),
        qualities: Vec::default(),
        two_handed: true,
        wearable: None,
        melee_damage: None,
        throw_damage: None,
        ranged_damage: None,
        is_ammo: None,
        need_ammo: None,
        color_from_material: Some(Material::Flesh),
    });
    if let Some(color) = appearance.body_color {
        body.with_colored(color)
    } else {
        body
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::Point;

    use crate::game::traits::{LooksLike, Name};
    use crate::game::units::tests::helpers::tester_girl;
    use crate::game::units::Player;

    use super::*;

    #[test]
    fn test_dead_body() {
        let unit = Player::new(tester_girl(), Point::default());
        let body = dead_body(unit.appearance());
        assert_eq!(body.name(), "dead gazan girl");
        assert_eq!(body.looks_like(), Sprite::Corpse);
        assert_eq!(body.size(), ItemSize::Huge);
        assert_eq!(body.proto().color_from_material, Some(Material::Flesh));
        assert_eq!(body.color(), unit.view().fg().unwrap())
    }
}
