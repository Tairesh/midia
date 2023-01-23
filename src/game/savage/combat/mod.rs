pub use hit::{HitCauses, HitParams, HitResult};
pub use melee::{melee_attack_unit, melee_smash_terrain, TerrainAttackResult, UnitAttackResult};

mod hit;
mod melee;
// TODO: distance attacks: bows, throwing, and melee weapons with distance > 0
