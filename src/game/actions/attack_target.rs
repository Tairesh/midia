use geometry::Point;

use crate::game::World;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type", content = "target")]
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

#[cfg(test)]
mod tests {
    use crate::game::world::tests::{add_monster, prepare_world};

    use super::*;

    #[test]
    fn test_attack_target_auto() {
        let mut world = prepare_world();

        let pos = Point::new(1, 1);
        assert_eq!(AttackTarget::auto(pos, &world), AttackTarget::terrain(pos));

        let npc = add_monster(&mut world, pos);
        assert_eq!(AttackTarget::auto(pos, &world), AttackTarget::avatar(npc));
    }

    #[test]
    fn test_attack_target_terrain_serialize() {
        let pos = Point::new(1, 1);
        let target = AttackTarget::terrain(pos);
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, r#"{"type":"terrain","target":{"x":1,"y":1}}"#);
        let deserialized: AttackTarget = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, target);
    }

    #[test]
    fn test_attack_target_avatar_serialize() {
        let mut world = prepare_world();
        let pos = Point::new(1, 1);
        let avatar = add_monster(&mut world, pos);
        let target = AttackTarget::avatar(avatar);
        let serialized = serde_json::to_string(&target).unwrap();
        assert_eq!(serialized, r#"{"type":"avatar","target":1}"#);
        let deserialized: AttackTarget = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, target);
    }
}
