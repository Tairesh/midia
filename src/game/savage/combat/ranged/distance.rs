#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangedDistance {
    Melee,
    Close,
    Medium,
    Far,
    Unreachable,
}

impl RangedDistance {
    pub fn define(distance: f64, weapon_distance: u8) -> Self {
        let distance = distance.round() as u8;
        if distance <= 1 {
            Self::Melee
        } else if distance <= weapon_distance {
            Self::Close
        } else if distance <= weapon_distance * 2 {
            Self::Medium
        } else if distance <= weapon_distance * 4 {
            Self::Far
        } else {
            Self::Unreachable
        }
    }

    pub fn modifier(self) -> i8 {
        match self {
            Self::Close => 0,
            Self::Medium => -1,
            Self::Melee | Self::Far => -2,
            Self::Unreachable => {
                unreachable!("Trying to calculate modifier for unreachable distance")
            }
        }
    }
}
