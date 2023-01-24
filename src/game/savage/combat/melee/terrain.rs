use crate::game::{Avatar, Terrain, TerrainInteract};

use super::fighting_roll;

const TERRAIN_PARRY: i8 = 2;

pub fn melee_smash_terrain(attacker: &Avatar, defender: &Terrain) -> TerrainAttackResult {
    let hit_roll = fighting_roll(attacker);
    if hit_roll >= TERRAIN_PARRY {
        let melee_damage = attacker.melee_damage();

        let damage = melee_damage
            .damage
            .roll(&attacker.personality.char_sheet, false, false);
        if damage >= defender.smash_toughness() {
            TerrainAttackResult::Success(damage)
        } else {
            TerrainAttackResult::Hit(damage)
        }
    } else {
        TerrainAttackResult::Miss
    }
}

pub enum TerrainAttackResult {
    Miss,
    Hit(u8),
    Success(u8),
}
