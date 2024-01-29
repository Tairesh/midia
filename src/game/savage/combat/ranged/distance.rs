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

#[cfg(test)]
mod tests {
    use geometry::Point;
    use test_case::test_case;

    use super::*;

    #[test_case(Point::new(0, 0), Point::new(0, 0), 15, RangedDistance::Melee)]
    #[test_case(Point::new(0, 0), Point::new(1, 0), 15, RangedDistance::Melee)]
    #[test_case(Point::new(0, 0), Point::new(1, 1), 15, RangedDistance::Melee)]
    #[test_case(Point::new(0, 0), Point::new(2, 1), 15, RangedDistance::Close)]
    #[test_case(Point::new(0, 0), Point::new(15, 0), 15, RangedDistance::Close)]
    #[test_case(Point::new(0, 0), Point::new(16, 0), 15, RangedDistance::Medium)]
    #[test_case(Point::new(0, 0), Point::new(30, 0), 15, RangedDistance::Medium)]
    #[test_case(Point::new(0, 0), Point::new(31, 0), 15, RangedDistance::Far)]
    #[test_case(Point::new(0, 0), Point::new(60, 0), 15, RangedDistance::Far)]
    #[test_case(Point::new(0, 0), Point::new(61, 0), 15, RangedDistance::Unreachable)]
    fn test_define(start: Point, target: Point, weapon_distance: u8, expected: RangedDistance) {
        assert_eq!(
            RangedDistance::define(start.distance(target), weapon_distance),
            expected
        );
    }
}
