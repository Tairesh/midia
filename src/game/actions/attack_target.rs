use geometry::Point;

use crate::game::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum AttackTarget {
    Terrain(Point),
    Avatar(usize),
}

impl AttackTarget {
    pub fn auto(pos: Point, world: &World) -> Self {
        let unit_id = world.map().get_tile(pos).units.iter().copied().next();
        if let Some(unit_id) = unit_id {
            Self::avatar(unit_id)
        } else {
            Self::terrain(pos)
        }
    }

    pub fn terrain(pos: Point) -> Self {
        Self::Terrain(pos)
    }

    pub fn avatar(id: usize) -> Self {
        Self::Avatar(id)
    }

    pub fn pos(self, world: &World) -> Point {
        match self {
            Self::Terrain(pos) => pos,
            Self::Avatar(id) => world.units().get_unit(id).pos(),
        }
    }
}
