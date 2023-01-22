use crate::game::{
    Avatar, BodySlot, Dice, DiceWithModifier, Item, Skill, Terrain, TerrainInteract,
};

use super::Wound;

pub enum UnitAttackResult {
    Miss,
    Hit(HitResult),
}

pub struct HitParams {
    pub damage: u8,
    pub penetration: u8,
    pub critical: bool,
}

impl HitParams {
    pub fn new(damage: u8, penetration: u8, critical: bool) -> Self {
        Self {
            damage,
            penetration,
            critical,
        }
    }
}

pub struct HitCauses {
    pub shock: bool,
    pub wounds: Vec<Wound>,
}

impl HitCauses {
    pub fn random_wounds(shock: bool, wounds: u8) -> Self {
        Self {
            shock,
            wounds: (0..wounds).map(|_| Wound::random()).collect(),
        }
    }
}

pub struct HitResult {
    pub params: HitParams,
    pub causes: HitCauses,
}

impl HitResult {
    pub fn new(params: HitParams, causes: HitCauses) -> Self {
        Self { params, causes }
    }

    #[cfg(test)]
    pub fn ultra_damage() -> Self {
        Self::new(
            HitParams::new(100, 100, true),
            HitCauses::random_wounds(true, 4),
        )
    }
}

pub enum TerrainAttackResult {
    Miss,
    Hit(u8),
    Success(u8),
}

/// Performs roll on hit
pub fn melee_hit_roll(attacker: &Avatar) -> u8 {
    let fightning_dice = attacker
        .personality
        .char_sheet
        .get_skill_with_modifiers(Skill::Fighting);
    if attacker.personality.char_sheet.wild_card {
        let wild_dice = DiceWithModifier::new(Dice::D6, fightning_dice.modifier());
        u8::max(fightning_dice.roll_explosive(), wild_dice.roll_explosive())
    } else {
        fightning_dice.roll_explosive()
    }
}

static TERRAIN_PARRY: u8 = 2;

pub fn melee_attack_terrain(attacker: &Avatar, defender: &Terrain) -> TerrainAttackResult {
    let hit_roll = melee_hit_roll(attacker);
    if hit_roll >= TERRAIN_PARRY {
        let delta = hit_roll - TERRAIN_PARRY;
        let critical = delta >= 4;

        let damage_params = attacker
            .wield
            .active_hand()
            .map(Item::melee_damage)
            .unwrap_or_default();

        let damage = damage_params
            .damage
            .roll(&attacker.personality.char_sheet, critical, false);
        if damage >= defender.smash_toughness() {
            TerrainAttackResult::Success(damage)
        } else {
            TerrainAttackResult::Hit(damage)
        }
    } else {
        TerrainAttackResult::Miss
    }
}

// TODO: attack terrains and items
// TODO: refactor this function to smaller ones
pub fn melee_attack_unit(attacker: &Avatar, defender: &Avatar) -> UnitAttackResult {
    // TODO: Attack of unarmed enemy while attacker is armed causes +2 to Fighting skill rolls
    // TODO: add +1 to hit for every ally
    let hit_roll = melee_hit_roll(attacker);

    let parry = defender.personality.char_sheet.parry();
    if hit_roll >= parry {
        let delta = hit_roll - parry;
        let critical = delta >= 4;

        let damage_params = attacker
            .wield
            .active_hand()
            .map(Item::melee_damage)
            .unwrap_or_default();
        let damage = damage_params
            .damage
            .roll(&attacker.personality.char_sheet, critical, true);
        let penetration = damage_params.penetration;

        let toughness = defender.personality.char_sheet.toughness();
        // TODO: attack random parts of the body
        let mut armor = defender.armor(BodySlot::Torso);
        armor = armor.saturating_sub(penetration);
        let mut total_damage = damage.saturating_sub(toughness + armor);

        let mut shock = false;
        let mut wounds = 0;
        if total_damage > 0 {
            // add wound if target already shocked
            if defender.personality.char_sheet.shock {
                wounds += 1;
            }
            shock = true;
            // add wound for every success
            while total_damage > 4 {
                wounds += 1;
                total_damage -= 4;
            }
        }

        UnitAttackResult::Hit(HitResult::new(
            HitParams::new(damage, penetration, critical),
            HitCauses::random_wounds(shock, wounds),
        ))
    } else {
        UnitAttackResult::Miss
    }
}

// TODO: special attacks: both hands, kick, agressive, etc.
// TODO: distance attacks: bows, throwing, and melee weapons with distance > 0
