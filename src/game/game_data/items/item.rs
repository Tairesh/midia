use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::assets::Sprite;

use super::{DamageValue, IsAmmoValue, ItemQuality, Material, NeedAmmoValue, WearableValue};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemPrototype {
    pub id: String,
    pub name: String,
    pub looks_like: Sprite,
    pub size: ItemSize,
    pub materials: HashSet<Material>,
    #[serde(default)]
    pub qualities: Vec<ItemQuality>,
    #[serde(default)]
    pub two_handed: bool,
    #[serde(default)]
    pub wearable: Option<WearableValue>,
    // TODO: color_from_material sounds stupid, probably should be removed
    #[serde(default)]
    pub color_from_material: Option<Material>,
    #[serde(default)]
    pub melee_damage: Option<DamageValue>,
    #[serde(default)]
    pub throw_damage: Option<DamageValue>,
    #[serde(default)]
    pub ranged_damage: Option<DamageValue>,
    #[serde(default)]
    pub need_ammo: Option<NeedAmmoValue>,
    #[serde(default)]
    pub is_ammo: Option<IsAmmoValue>,
}
