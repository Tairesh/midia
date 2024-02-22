use geometry::{Point, DIR8};
use pathfinding::directed::astar::astar as astar_lib;
use smallvec::SmallVec;

use crate::game::map::{Map, Passage, TerrainInteract};

pub fn astar(map: &Map, start: Point, end: Point) -> Option<(Vec<Point>, u32)> {
    astar_lib(
        &start,
        |p| {
            DIR8.iter()
                .filter_map(|d| {
                    let point = *p + *d;
                    if let Some(tile) = map.get_tile_opt(point) {
                        return match tile.passage() {
                            Passage::Impassable => None,
                            Passage::Passable(cost) => Some((point, cost)),
                            Passage::TemporaryImpassable(unit_id) => {
                                if unit_id == 0 {
                                    Some((point, 100))
                                } else {
                                    None
                                }
                            }
                        };
                    }
                    None
                })
                .collect::<SmallVec<[_; 8]>>()
        },
        |p| p.square_distance(end),
        |p| *p == end,
    )
}

#[cfg(test)]
mod test {
    use crate::game::map::terrains::{Boulder, BoulderSize, Dirt, DirtVariant};
    use crate::game::world::tests::prepare_world;

    use super::{astar, Point, TerrainInteract};

    #[test]
    fn test_line() {
        let world = prepare_world();
        let points = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(0, 3),
        ];
        for point in &points {
            world.map().get_tile_mut(*point).terrain = Dirt::new(DirtVariant::Dirt1).into();
        }
        let result = astar(&world.map(), Point::new(0, 0), Point::new(0, 3));
        assert!(result.is_some());
        let (result, cost) = result.unwrap();
        assert_eq!(result, points);
        assert_eq!(cost, 30);
    }

    #[test]
    fn test_no_path() {
        let world = prepare_world();
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
            world.map().get_tile_mut(point).terrain = Boulder::new(BoulderSize::Huge).into();
        }

        let result = astar(&world.map(), Point::new(2, 0), Point::new(0, 0));
        assert!(result.is_none());
    }

    #[test]
    fn test_move_around() {
        let world = prepare_world();
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
            world.map().get_tile_mut(point).terrain = Boulder::new(BoulderSize::Huge).into();
        }

        let result = astar(&world.map(), Point::new(2, 0), Point::new(0, 0));
        assert!(result.is_some());
        let (result, _cost) = result.unwrap();
        assert_eq!(
            result,
            vec![
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
                Point { x: 4, y: -1 },
                Point { x: 3, y: -2 },
                Point { x: 2, y: -2 },
                Point { x: 1, y: -2 },
                Point { x: 0, y: -1 },
                Point { x: 0, y: 0 },
            ]
        );
    }

    // TODO: add more tests
}
