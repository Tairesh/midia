pub use terrain::{melee_smash_terrain, TerrainMeleeAttackResult};
pub use unit_simple::{melee_attack_unit, UnitMeleeAttackResult};

use crate::game::{Avatar, Skill};

mod terrain;
mod unit_simple;

fn fighting_roll(attacker: &Avatar) -> i8 {
    attacker
        .personality
        .char_sheet
        .roll_skill(Skill::Fighting, attacker.melee_damage().attack_modifier)
        .total
}
