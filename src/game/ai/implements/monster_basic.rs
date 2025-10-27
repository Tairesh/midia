use std::collections::HashMap;

use rand::Rng;
use roguemetry::{Direction, Point};
use serde::{Deserialize, Serialize};

use crate::game::{
    actions::implements::{Melee, Walk},
    Action, ActionType, Avatar, Map, World,
};

use super::super::{pathfinding::astar, AIImpl};

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
        let player_pos = world.player().pos();
        let unit = world.units.get_unit(unit_id).as_monster()?;
        let pos = unit.pos();
        let distance = pos.distance_to(player_pos).floor() as u32;
        let sight_range = unit.char_sheet().sight_range();
        // TODO: use World's rng instead of thread_rng
        // TODO: check if the player is visible
        // TODO: add aggro state and periodic Notice roll
        if distance > sight_range {
            return Action::new(
                unit_id,
                Walk::new(Direction::random(&mut rand::rng(), false)),
                world,
            )
            .ok();
        }

        let attack = Action::new(unit_id, Melee::new(player_pos, world), world);
        if let Ok(action) = attack {
            return Some(action);
        }

        let regenerate = if let Some(path) = self.selected_pathes.get(&unit_id) {
            !path.check(&world.map, pos, player_pos)
        } else {
            true
        };
        if regenerate {
            let new_path = SelectedPath::new(&world.map, pos, player_pos);
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
                return Action::new(unit_id, Walk::new(pos.direction_to(next_pos)), world).ok();
            }
        }

        Action::new(unit_id, Walk::new(pos.direction_to(player_pos)), world).ok()
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::{Direction, Point};

    use crate::game::actions::AttackTarget;
    use crate::game::map::terrains::{Boulder, BoulderSize, Dirt, DirtVariant};
    use crate::game::world::tests::{add_monster, boulder, dirt, prepare_world};
    use crate::game::{Avatar, Terrain};

    use super::*;

    #[test]
    fn test_monster_walk_to_player() {
        let mut world = prepare_world();
        for i in 0..2 {
            world.map.get_tile_mut(Point::new(i, 0)).terrain = dirt();
        }
        let npc = add_monster(&mut world, Point::new(2, 0));
        world.plan_test();

        let npc = world.units.get_unit(npc).as_monster().unwrap();
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

        let npc = world.units.get_unit(npc).as_monster().unwrap();
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
        world.map.get_tile_mut(Point::new(2, 0)).terrain = boulder();
        world.map.get_tile_mut(Point::new(2, 1)).terrain = boulder();
        world.map.get_tile_mut(Point::new(2, -1)).terrain = boulder();
        world.map.get_tile_mut(Point::new(3, -1)).terrain = boulder();
        world.map.get_tile_mut(Point::new(3, 1)).terrain = dirt();
        world.map.get_tile_mut(Point::new(2, 2)).terrain = dirt();
        world.map.get_tile_mut(Point::new(1, 2)).terrain = dirt();
        world.map.get_tile_mut(Point::new(1, 1)).terrain = dirt();
        world.map.get_tile_mut(Point::new(1, 0)).terrain = dirt();
        let npc = add_monster(&mut world, Point::new(3, 0));
        world.plan_test();

        let npc = world.units.get_unit(npc).as_monster().unwrap();
        assert!(npc.action().is_some());
        let action = npc.action().unwrap();
        if let ActionType::Walk(walk) = action.typ {
            assert_eq!(Direction::South, walk.dir());
        } else {
            panic!("Unexpected monster action: {:?}", action.typ);
        }
    }
}
