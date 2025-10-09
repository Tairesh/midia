use pathfinding::directed::astar::astar as astar_lib;
use roguemetry::{Point, DIR8};
use smallvec::SmallVec;

use crate::game::map::{Map, Passage, TerrainInteract};

pub fn astar(map: &Map, start: Point, end: Point) -> Option<(Vec<Point>, u32)> {
    astar_lib(
        &start,
        |p| {
            DIR8.iter()
                .filter_map(|d| {
                    let point = *p + *d;
                    map.get_passage_cost(point).map(|cost| (point, cost))
                })
                .collect::<SmallVec<[_; 8]>>()
        },
        |p| p.square_distance_to(end),
        |p| *p == end,
    )
}

#[cfg(test)]
mod test {
    use crate::game::map::terrains::{Boulder, BoulderSize, Dirt, DirtVariant};
    use crate::game::world::tests::{boulder, dirt, prepare_world};
    use crate::game::Terrain;

    use super::{astar, Point};

    #[test]
    fn test_line() {
        let mut world = prepare_world();
        let points = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ];
        for point in &points {
            world.map.get_tile_mut(*point).terrain = dirt();
        }
        let result = astar(&world.map, Point::new(0, 0), Point::new(0, 3));
        assert!(result.is_some());
        let (result, cost) = result.unwrap();
        assert_eq!(result, points);
        assert_eq!(cost, 30);
    }

    #[test]
    fn test_no_path() {
        let mut world = prepare_world();
        let blocked = vec![
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(1, -1),
            Point::new(2, -1),
            Point::new(2, 1),
            Point::new(3, 0),
            Point::new(3, 1),
            Point::new(3, -1),
        ];
        for point in blocked {
            world.map.get_tile_mut(point).terrain = boulder();
        }

        let result = astar(&world.map, Point::new(2, 0), Point::new(0, 0));
        assert!(result.is_none());
    }

    #[test]
    fn test_move_around() {
        let mut world = prepare_world();
        let blocked = vec![
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(1, -1),
            Point::new(2, -1),
            Point::new(2, 1),
            Point::new(3, 1),
            Point::new(3, -1),
        ];
        for point in blocked {
            world.map.get_tile_mut(point).terrain = boulder();
        }
        let free = vec![
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 1),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
            Point::new(0, 1),
            Point::new(0, 0),
        ];
        for point in &free {
            world.map.get_tile_mut(*point).terrain = dirt();
        }

        let result = astar(&world.map, Point::new(2, 0), Point::new(0, 0));
        assert!(result.is_some());
        let (result, _cost) = result.unwrap();
        assert_eq!(result, free);
    }

    // TODO: add more tests
}
