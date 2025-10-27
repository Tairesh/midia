use crate::game::units::Weapon;
use crate::lang::a;
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
    pub fn name(self) -> &'static str {
        match self {
            AttackType::Melee => "melee",
            AttackType::Throw => "throw",
            AttackType::Shoot => "shoot from",
        }
    }

    /// E.g. "shoot from[ a bow]", "throw[ a spear]", "attack with[ a sword]"
    pub fn verb_a(self, with_s: bool, weapon: &Weapon) -> String {
        let weapon_name = a(&weapon.name);
        if with_s {
            match self {
                AttackType::Melee => format!("attacks with {weapon_name}"),
                AttackType::Throw => format!("throws {weapon_name}"),
                AttackType::Shoot => {
                    if let Some(ammo) = &weapon.ammo_name {
                        format!("shoots the {ammo} from {weapon_name}")
                    } else {
                        format!("shoots from {weapon_name}")
                    }
                }
            }
        } else {
            match self {
                AttackType::Melee => format!("attack with {weapon_name}"),
                AttackType::Throw => format!("throw {weapon_name}"),
                AttackType::Shoot => {
                    if let Some(ammo) = &weapon.ammo_name {
                        format!("shoot the {ammo} from {weapon_name}")
                    } else {
                        format!("shoot from {weapon_name}")
                    }
                }
            }
        }
    }
}
