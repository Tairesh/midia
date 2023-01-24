pub use hit::HitResult;
pub use melee::{
    melee_attack_unit, melee_smash_terrain, TerrainMeleeAttackResult, UnitMeleeAttackResult,
};
pub use ranged::{throw_attack_unit, RangedDistance, UnitRangedAttackResult};

mod hit;
mod melee;
mod ranged;
// TODO: distance attacks: bows, throwing, and melee weapons with distance > 0
