use crate::game::{Avatar, Item, Skill, Terrain, TerrainInteract};

const TERRAIN_PARRY: u8 = 2;

pub fn melee_smash_terrain(attacker: &Avatar, defender: &Terrain) -> TerrainAttackResult {
    let hit_roll = attacker.personality.char_sheet.roll_skill(Skill::Fighting);
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

pub enum TerrainAttackResult {
    Miss,
    Hit(u8),
    Success(u8),
}
