#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Wound {
    Groin,
    LeftArm,
    RightArm,
    BrokenGuts,
    BatteredGuts,
    BustedGuts,
    LeftLeg,
    RightLeg,
    HideousScar,
    LeftEye,
    RightEye,
    BrainDamage,
}
