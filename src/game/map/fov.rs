use std::collections::HashSet;

use geometry::Point;

use crate::game::Map;

// it is basically just copy-pasted from https://github.com/amethyst/bracket-lib/blob/master/bracket-pathfinding/src/field_of_view/recursive_shadowcasting.rs

#[derive(Debug)]
pub struct Fov {
    visible: HashSet<Point>,
}

impl Fov {
    pub fn new() -> Self {
        Self {
            visible: HashSet::default(),
        }
    }

    pub fn set_visible(&mut self, set: HashSet<Point>) {
        self.visible = set;
    }

    pub fn visible(&self) -> &HashSet<Point> {
        &self.visible
    }
}

impl Default for Fov {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FovMap {
    fn is_transparent(&self, point: Point) -> bool;
}

pub fn field_of_view_set(center: Point, range: i32, fov_check: &Map) -> HashSet<Point> {
    /* N, NE, E, SE, S, SW, W, NW */
    const SECTORS: [(i32, i32); 8] = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    visible_points.insert(center);

    let r2 = range * range;

    // Add visibility for every 45 degree line:
    let mut visibility_per_sector = [false; 8];
    for (i, (dx, dy)) in SECTORS.iter().enumerate() {
        let mut current = center;
        loop {
            current = Point::new(current.x + dx, current.y + dy);
            let x2 = current.x - center.x;
            let x2 = x2 * x2;
            let y2 = current.y - center.y;
            let y2 = y2 * y2;
            if x2 + y2 > r2 {
                break;
            }

            visible_points.insert(current);
            if !fov_check.is_transparent(current) {
                break;
            }
            visibility_per_sector[i] = true;
        }
    }

    let mut scanner = ScanFovData {
        center,
        range_2: r2,
        fov_check,
        visible_points: &mut visible_points,
    };
    if visibility_per_sector[0] {
        scanner.scan_N2NW(1, 0., 1.);
        scanner.scan_N2NE(1, 0., 1.);
    }

    if visibility_per_sector[2] {
        scanner.scan_E2NE(1, 0., 1.);
        scanner.scan_E2SE(1, 0., 1.);
    }

    if visibility_per_sector[4] {
        scanner.scan_S2SE(1, 0., 1.);
        scanner.scan_S2SW(1, 0., 1.);
    }

    if visibility_per_sector[6] {
        scanner.scan_W2SW(1, 0., 1.);
        scanner.scan_W2NW(1, 0., 1.);
    }

    visible_points
}

struct ScanFovData<'a> {
    center: Point,
    range_2: i32,
    fov_check: &'a dyn FovMap,
    visible_points: &'a mut HashSet<Point>,
}

#[allow(non_snake_case)]
impl ScanFovData<'_> {
    fn is_transparent(&self, point: Point) -> bool {
        self.fov_check.is_transparent(point)
    }

    fn distance_to_center(&self, point: Point) -> i32 {
        point.square_distance(self.center) as i32
    }

    fn insert_visible_for_vertical(&mut self, point: Point) -> bool {
        let mut is_visible = self.is_transparent(point);

        if self.distance_to_center(point) <= self.range_2 {
            if point.x != self.center.x {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn insert_visible_for_horizontal(&mut self, point: Point) -> bool {
        let mut is_visible = self.is_transparent(point);

        if self.distance_to_center(point) <= self.range_2 {
            if self.center.y != point.y {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn scan_N2NE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance);

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;

        let end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_N2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_N2NW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance);

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;

        let end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_N2NW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance);

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;

        let end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_S2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance);

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;

        let end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_S2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2SE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance, 0);

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;

        let end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2SE(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_E2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2NE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance, 0);

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;

        let end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2NE(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_E2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2SW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance, 0);

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;

        let end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2SW(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_W2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2NW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance, 0);

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;

        let end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;

        let mut last_visible = self.is_transparent(current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_W2NW(distance + 1, start_slope, end_slope);
        }
    }
}
