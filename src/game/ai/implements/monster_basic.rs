use std::collections::HashMap;

use geometry::Point;
use serde::{Deserialize, Serialize};

use crate::game::{
    actions::implements::{Melee, Walk},
    Action, ActionType, Avatar, Map, World,
};

use super::super::{pathfinding::astar, AIImpl, AI};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SelectedPath {
    path: Vec<Point>,
    cost: u32,
    to: Point,
}

impl SelectedPath {
    pub fn new(map: &Map, from: Point, to: Point) -> Option<Self> {
        let (path, cost) = astar(map, from, to)?;
        Some(Self { path, cost, to })
    }

    pub fn check(&self, map: &Map, from: Point, to: Point) -> bool {
        if self.to != to {
            return false;
        }

        let mut point_found = false;
        for point in &self.path {
            if *point == from {
                point_found = true;
            }
            if let Some(tile) = map.get_tile_opt(*point) {
                if !tile.is_passable() {
                    return false;
                }
            } else {
                return false;
            }
        }

        point_found
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicMonsterAI {
    selected_pathes: HashMap<usize, SelectedPath>,
}

impl BasicMonsterAI {
    pub fn new() -> Self {
        Self {
            selected_pathes: HashMap::new(),
        }
    }
}

impl AIImpl for BasicMonsterAI {
    fn plan(&mut self, unit_id: usize, world: &World) -> Option<Action> {
        let units = world.units();
        let unit = units.get_unit(unit_id).as_monster()?;
        let pos = unit.pos();
        let player_pos = units.player().pos();
        let attack = Action::new(unit_id, Melee::new(player_pos, world).into(), world);
        if let Ok(action) = attack {
            return Some(action);
        }

        let regenerate = if let Some(path) = self.selected_pathes.get(&unit_id) {
            !path.check(&world.map(), pos, player_pos)
        } else {
            true
        };
        if regenerate {
            let new_path = SelectedPath::new(&world.map(), pos, player_pos);
            if let Some(new_path) = new_path {
                self.selected_pathes.insert(unit_id, new_path);
            } else {
                self.selected_pathes.remove(&unit_id);
            }
        }
        if let Some(path) = self.selected_pathes.get(&unit_id) {
            let pos_index = path.path.iter().position(|&p| p == pos)?;
            if pos_index + 1 < path.path.len() {
                let next_pos = path.path[pos_index + 1];
                return Action::new(unit_id, Walk::new(pos.dir_to(next_pos)).into(), world).ok();
            }
        }

        Action::new(unit_id, Walk::new(pos.dir_to(player_pos)).into(), world).ok()
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::actions::implements::Walk;
    use crate::game::actions::AttackTarget;
    use crate::game::map::terrains::{Boulder, BoulderSize};
    use crate::game::world::tests::{add_monster, prepare_world};
    use crate::game::Avatar;

    use super::*;

    #[test]
    fn test_monster_walk_to_player() {
        let mut world = prepare_world();
        let npc = add_monster(&mut world, Point::new(7, 0));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        if let ActionType::Walk(walk) = action.typ {
            assert_eq!(Direction::West, walk.dir());
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }

    #[test]
    fn test_monster_attack_player() {
        let mut world = prepare_world();
        let npc = add_monster(&mut world, Point::new(1, 0));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        if let ActionType::Melee(melee) = action.typ {
            match melee.target() {
                AttackTarget::Avatar(id) => assert_eq!(0, id),
                _ => panic!("Unexpected target: {:?}", melee.target()),
            }
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }

    #[test]
    fn test_monster_finds_path() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(2, 0)).terrain = Boulder::new(BoulderSize::Huge).into();
        world.map().get_tile_mut(Point::new(2, 1)).terrain = Boulder::new(BoulderSize::Huge).into();
        world.map().get_tile_mut(Point::new(2, -1)).terrain =
            Boulder::new(BoulderSize::Huge).into();
        world.map().get_tile_mut(Point::new(3, -1)).terrain =
            Boulder::new(BoulderSize::Huge).into();
        let npc = add_monster(&mut world, Point::new(3, 0));
        world.plan_test();

        let units = world.units();
        let npc = units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        if let ActionType::Walk(walk) = action.typ {
            assert_eq!(Direction::South, walk.dir());
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }
}
