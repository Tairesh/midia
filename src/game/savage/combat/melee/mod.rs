pub use terrain::{melee_smash_terrain, TerrainMeleeAttackResult};
pub use unit_simple::{melee_attack_unit, UnitMeleeAttackResult};

use crate::game::{AttackType, Avatar, Fighter, Skill};

mod terrain;
mod unit_simple;

fn fighting_roll(attacker: &dyn Fighter) -> i8 {
    attacker
        .as_avatar()
        .char_sheet()
        .roll_skill(
            Skill::Fighting,
            attacker
                .weapon(AttackType::Melee)
                .unwrap()
                .damage
                .attack_modifier,
        )
        .total
}
