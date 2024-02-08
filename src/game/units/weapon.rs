use crate::game::DamageValue;

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub damage: DamageValue,
}
