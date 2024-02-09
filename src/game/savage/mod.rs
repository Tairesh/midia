pub use attributes::{Attribute, Attributes};
pub use charsheet::CharSheet;
pub use combat::{
    melee_attack_unit, melee_smash_terrain, ranged_attack_unit, AttackType, HitResult,
    RangedDistance, TerrainMeleeAttackResult, UnitMeleeAttackResult, UnitRangedAttackResult,
};
pub use damage::{Damage, DamageDice, DamageRollResult, DamageType};
pub use dices::{AttrLevel, Dice, DiceWithModifier, RollResult, SkillLevel};
pub use skills::{Skill, Skills};
pub use wound::Wound;

mod attributes;
mod charsheet;
mod combat;
mod damage;
mod dices;
mod skills;
mod wound;

/// Default attack cost in ticks
pub const ATTACK_MOVES: u32 = 10;
