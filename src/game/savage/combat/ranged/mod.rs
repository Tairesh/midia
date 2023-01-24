use std::collections::HashMap;

use geometry::Point;

pub use distance::RangedDistance;
pub use throw::throw_attack_unit;

use super::HitResult;

mod distance;
mod throw;

pub enum UnitRangedAttackResult {
    InnocentBystander(usize, HitResult),
    Miss,
    Hit(HitResult),
    #[allow(dead_code)]
    Explosion(HashMap<usize, HitResult>, HashMap<Point, u8>),
    Impossible,
}
