use std::collections::HashSet;

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::colors::Colors;
use crate::game::races::BodySlot;
use crate::game::savage::Damage;
use crate::game::{Attribute, CharSheet, DamageDice, Dice, Item};

// TODO: move this to subfolder

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    Tool,
    Weapon,
    Book,
    Corpse,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
    /// can store items
    Container {
        volume: u8,
        #[serde(default)]
        for_ammo: HashSet<AmmoType>,
    },
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
pub struct DamageValue {
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
    #[serde(default)]
    pub minimum_strength: Option<Dice>,
}

fn damage_types(item: &Item) -> HashSet<DamageType> {
    if let Some(melee_damage) = item.proto().melee_damage.as_ref() {
        melee_damage.damage_types.clone()
    } else if let Some(throw_damage) = item.proto().throw_damage.as_ref() {
        throw_damage.damage_types.clone()
    } else if item
        .proto()
        .materials
        .iter()
        .copied()
        .any(Material::is_hard)
    {
        HashSet::from([DamageType::Blunt])
    } else {
        HashSet::new()
    }
}

impl DamageValue {
    pub fn zero() -> Self {
        Self {
            damage: Damage {
                dices: vec![],
                attribute: None,
                modifier: 0,
                crit_dice: None,
            },
            damage_types: HashSet::new(),
            distance: 0,
            penetration: 0,
            attack_modifier: 0,
            parry_modifier: 0,
            minimum_strength: None,
        }
    }

    pub fn improvised_melee(item: &Item) -> Self {
        Self {
            damage: Damage {
                dices: match item.size() {
                    ItemSize::Tiny => vec![],
                    ItemSize::Small => vec![DamageDice::D4],
                    ItemSize::Medium => vec![DamageDice::D6],
                    ItemSize::Large | ItemSize::Huge => vec![DamageDice::D8],
                },
                attribute: Some(Attribute::Strength),
                modifier: 0,
                crit_dice: None,
            },
            damage_types: damage_types(item),
            distance: 0,
            penetration: 0,
            attack_modifier: -1,
            parry_modifier: -1,
            minimum_strength: match item.size() {
                ItemSize::Tiny => None,
                ItemSize::Small => Some(Dice::D4),
                ItemSize::Medium => Some(Dice::D6),
                ItemSize::Large => Some(Dice::D8),
                ItemSize::Huge => Some(Dice::D12),
            },
        }
    }

    pub fn improvised_throw(item: &Item) -> Option<Self> {
        if item.size() == ItemSize::Huge {
            return None;
        }
        Some(Self {
            distance: match item.size() {
                ItemSize::Tiny | ItemSize::Small => 3,
                ItemSize::Medium => 2,
                ItemSize::Large => 1,
                ItemSize::Huge => unreachable!(),
            },
            ..Self::improvised_melee(item)
        })
    }

    pub fn roll(
        &self,
        char_sheet: &CharSheet,
        critical: bool,
        explosive: bool,
    ) -> DamageRollResult {
        let damage_type = self
            .damage_types
            .iter()
            .copied()
            .choose(&mut rand::thread_rng());
        if let Some(damage_type) = damage_type {
            DamageRollResult::new(
                self.damage
                    .roll(char_sheet, critical, explosive, self.minimum_strength),
                damage_type,
                self.penetration,
            )
        } else {
            DamageRollResult::empty()
        }
    }
}

pub struct DamageRollResult {
    pub damage: u8,
    pub damage_type: Option<DamageType>,
    pub penetration: u8,
}

impl DamageRollResult {
    pub fn new(damage: u8, damage_type: DamageType, penetration: u8) -> Self {
        Self {
            damage,
            penetration,
            damage_type: Some(damage_type),
        }
    }

    pub fn empty() -> Self {
        Self {
            damage: 0,
            damage_type: None,
            penetration: 0,
        }
    }
}

impl Default for DamageRollResult {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmmoType {
    Arrow,
    Bolt,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AmmoValue {
    pub typ: HashSet<AmmoType>,
    pub damage_modifier: DamageModifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DamageModifier {
    #[serde(default)]
    pub damage: i8,
    #[serde(default)]
    pub penetration: u8,
    #[serde(default)]
    pub damage_dice: Option<DamageDice>,
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
    pub qualities: Vec<ItemQuality>,
    #[serde(default)]
    pub two_handed_tool: bool,
    #[serde(default)]
    pub wearable: Option<WearableValue>,
    #[serde(default)]
    pub color_from_material: Option<Material>,
    #[serde(default)]
    pub melee_damage: Option<DamageValue>,
    #[serde(default)]
    pub throw_damage: Option<DamageValue>,
    #[serde(default)]
    pub ranged_damage: Option<DamageValue>,
    #[serde(default)]
    pub ammo_types: HashSet<AmmoType>,
    #[serde(default)]
    pub ammo: Option<AmmoValue>,
}
