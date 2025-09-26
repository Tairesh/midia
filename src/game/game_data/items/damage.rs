use std::collections::HashSet;

use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};

use crate::game::{
    Attribute, CharSheet, Damage, DamageDice, DamageRollResult, DamageType, Dice, Item,
};

use super::{ItemSize, Material};

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

    pub fn strength(damage_type: DamageType) -> Self {
        Self {
            damage: Damage {
                dices: Vec::new(),
                attribute: Some(Attribute::Strength),
                modifier: 0,
                crit_dice: None,
            },
            damage_types: HashSet::from([damage_type]),
            distance: 0,
            penetration: 0,
            attack_modifier: 0,
            parry_modifier: 0,
            minimum_strength: None,
        }
    }

    pub fn simple(dice: DamageDice, damage_type: DamageType) -> Self {
        Self {
            damage: Damage {
                dices: vec![dice],
                attribute: Some(Attribute::Strength),
                modifier: 0,
                crit_dice: None,
            },
            damage_types: HashSet::from([damage_type]),
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
            // TODO: use World's rng instead of thread_rng
            .choose(&mut rand::rng());
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
