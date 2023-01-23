use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::races::BodySlot;
use crate::game::savage::Damage;

// TODO: move this to subfolder

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    Tool,
    Weapon,
    Book,
    Corpse,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemQuality {
    /// can be used for digging
    Dig,
    /// can be used for butchering corpses
    Butch,
    /// can be used for cutting things
    Cut,
    /// can be used for chopping trees
    Chop,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WearLayer {
    Inner,
    Middle,
    Outer,
    Clipped,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DamageType {
    Blunt,
    Pierce,
    Slash,
    Fire,
    Cold,
    Acid,
    Electric,
    Poison,
    Magic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeleeDamageValue {
    pub damage: Damage,
    pub damage_types: HashSet<DamageType>,
    #[serde(default)]
    pub distance: u8,
    #[serde(default)]
    pub penetration: u8,
    #[serde(default)]
    pub attack_modifier: i8,
    #[serde(default)]
    pub parry_modifier: i8,
    // TODO: minumum strength
}

impl MeleeDamageValue {
    pub fn zero() -> Self {
        Self {
            damage: Damage {
                dices: vec![],
                attribute: None,
                modifier: 0,
            },
            damage_types: HashSet::new(),
            distance: 0,
            penetration: 0,
            attack_modifier: 0,
            parry_modifier: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WearableValue {
    pub layer: WearLayer,
    pub armor: u8,
    pub variants: Vec<HashSet<BodySlot>>,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemSize {
    /// like a coin
    Tiny,
    /// like a dagger
    Small,
    /// like a sword
    Medium,
    /// like a polearm
    Large,
    /// like a boulder
    Huge,
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
#[serde(rename_all = "snake_case")]
pub enum Material {
    Cloth,
    Wool,
    Leather,
    Wood,
    Stone,
    Iron,
    Steel,
    Obsidian,
    Demonite,
    LapisLazuli,
    Bone,
    Flesh,
    Plant,
    Paper,
}

impl Material {
    pub fn is_hard(self) -> bool {
        matches!(
            self,
            Self::Wood
                | Self::Stone
                | Self::Iron
                | Self::Steel
                | Self::Obsidian
                | Self::Demonite
                | Self::LapisLazuli
                | Self::Bone
        )
    }
}

impl From<Material> for Color {
    fn from(value: Material) -> Self {
        match value {
            Material::Cloth | Material::Wool => Colors::CLOTH,
            Material::Leather => Colors::LEATHER,
            Material::Wood => Colors::WOOD,
            Material::Stone => Colors::STONE,
            Material::Iron | Material::Steel => Colors::METAL,
            Material::Obsidian => Colors::OBSIDIAN,
            Material::Demonite => Colors::DEMONITE,
            Material::LapisLazuli => Colors::LAPIS_LAZULI,
            Material::Bone => Colors::BONE,
            Material::Flesh => Colors::RED,
            Material::Plant => Colors::PLANT,
            Material::Paper => Colors::LIGHT_SEPIA,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub looks_like: String,
    pub size: ItemSize,
    pub materials: HashSet<Material>,
    #[serde(default)]
    pub tags: HashSet<ItemTag>,
    #[serde(default)]
    pub qualities: HashSet<ItemQuality>,
    #[serde(default)]
    pub two_handed_tool: bool,
    #[serde(default)]
    pub wearable: Option<WearableValue>,
    #[serde(default)]
    pub melee_damage: Option<MeleeDamageValue>,
    #[serde(default)]
    pub color_from_material: Option<Material>,
}
