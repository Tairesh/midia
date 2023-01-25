use crate::game::{Avatar, DamageType, Terrain, TerrainInteract};

use super::fighting_roll;

const TERRAIN_PARRY: i8 = 2;

pub fn melee_smash_terrain(attacker: &Avatar, defender: &Terrain) -> TerrainMeleeAttackResult {
    let hit_roll = fighting_roll(attacker);
    if hit_roll >= TERRAIN_PARRY {
        let melee_damage = attacker.melee_damage();
        let damage = melee_damage.roll(&attacker.personality.char_sheet, false, false);

        if damage.damage_type.is_some()
            && damage.damage_type.unwrap() == DamageType::Blunt
            && damage.damage >= defender.smash_toughness()
        {
            TerrainMeleeAttackResult::Success(damage.damage)
        } else {
            TerrainMeleeAttackResult::Hit(damage.damage)
        }
    } else {
        TerrainMeleeAttackResult::Miss
    }
}

pub enum TerrainMeleeAttackResult {
    Miss,

    Hit(u8),
    Success(u8),
}
