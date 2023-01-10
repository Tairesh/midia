use std::collections::HashSet;

use geometry::Point;

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
