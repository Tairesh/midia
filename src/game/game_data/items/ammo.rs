use serde::{Deserialize, Serialize};

use crate::game::DamageDice;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AmmoType {
    Arrow,
    Bolt,
    Rock,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AmmoDamageModifier {
    #[serde(default)]
    pub damage: i8,
    #[serde(default)]
    pub penetration: u8,
    #[serde(default)]
    pub damage_dice: Option<DamageDice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAmmoValue {
    pub typ: AmmoType,
    #[serde(default)]
    pub damage_modifier: AmmoDamageModifier,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct NeedAmmoValue {
    pub typ: AmmoType,
    pub capacity: u8,
    pub reload: u8,
}
