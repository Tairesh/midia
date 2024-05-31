pub use hit::HitResult;
pub use melee::{
    melee_attack_unit, melee_smash_terrain, TerrainMeleeAttackResult, UnitMeleeAttackResult,
};
pub use ranged::{ranged_attack_unit, RangedDistance, UnitRangedAttackResult};

mod hit;
mod melee;
mod ranged;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AttackType {
    Melee,
    Throw,
    Shoot,
}

impl AttackType {
    /// E.g. "shoot from[ a bow]", "throw[ a spear]", "attack with[ a sword]"
    pub fn verb_a(self, with_s: bool) -> &'static str {
        if with_s {
            match self {
                AttackType::Melee => "attacks with",
                AttackType::Throw => "throws",
                AttackType::Shoot => "shoots from",
            }
        } else {
            match self {
                AttackType::Melee => "attack with",
                AttackType::Throw => "throw",
                AttackType::Shoot => "shoot from",
            }
        }
    }
}
