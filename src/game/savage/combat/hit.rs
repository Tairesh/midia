use crate::game::Wound;

pub struct HitResult {
    pub params: HitParams,
    pub causes: HitCauses,
}

impl HitResult {
    pub fn new(params: HitParams, causes: HitCauses) -> Self {
        Self { params, causes }
    }

    #[cfg(test)]
    pub fn ultra_damage() -> Self {
        Self::new(
            HitParams::new(100, 100, true),
            HitCauses::random_wounds(true, 4),
        )
    }
}

pub struct HitParams {
    pub damage: u8,
    pub penetration: u8,
    pub critical: bool,
}

impl HitParams {
    pub fn new(damage: u8, penetration: u8, critical: bool) -> Self {
        Self {
            damage,
            penetration,
            critical,
        }
    }
}

pub struct HitCauses {
    pub shock: bool,
    pub wounds: Vec<Wound>,
}

impl HitCauses {
    pub fn random_wounds(shock: bool, wounds: u8) -> Self {
        Self {
            shock,
            wounds: (0..wounds).map(|_| Wound::random()).collect(),
        }
    }
}
