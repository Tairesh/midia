use crate::game::{Avatar, Terrain, TerrainInteract};

use super::fighting_roll;

const TERRAIN_PARRY: i8 = 2;

pub fn melee_smash_terrain(attacker: &Avatar, defender: &Terrain) -> TerrainMeleeAttackResult {
    let hit_roll = fighting_roll(attacker);
    if hit_roll >= TERRAIN_PARRY {
        let melee_damage = attacker.melee_damage();

        let damage = melee_damage
            .damage
            .roll(&attacker.personality.char_sheet, false, false);
        if damage >= defender.smash_toughness() {
            TerrainMeleeAttackResult::Success(damage)
        } else {
            TerrainMeleeAttackResult::Hit(damage)
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
