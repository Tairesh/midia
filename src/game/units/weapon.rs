use crate::game::DamageValue;

#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub ammo_name: Option<String>,
    pub damage: DamageValue,
}
