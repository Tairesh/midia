pub use terrain::{melee_smash_terrain, TerrainAttackResult};
pub use unit_simple::{melee_attack_unit, UnitAttackResult};

use crate::game::{Avatar, Skill};

mod terrain;
mod unit_simple;

fn fighting_roll(attacker: &Avatar) -> i8 {
    attacker
        .personality
        .char_sheet
        .roll_skill(
            Skill::Fighting,
            if let Some(weapon) = attacker.wield.active_hand() {
                weapon.melee_damage().attack_modifier
            } else {
                0
            },
        )
        .total
}
