pub use hit::HitResult;
pub use melee::{
    melee_attack_unit, melee_smash_terrain, TerrainMeleeAttackResult, UnitMeleeAttackResult,
};
pub use ranged::{throw_attack_unit, RangedDistance, UnitRangedAttackResult};

mod hit;
mod melee;
mod ranged;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AttackType {
    Melee,
    Throw,
    Fire,
}
