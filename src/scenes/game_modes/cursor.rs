use geometry::Point;
use tetra::graphics::Color;

use crate::game::traits::LooksLike;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorType {
    Select,
    Fill,
}

impl LooksLike for CursorType {
    fn looks_like(&self) -> &'static str {
        match self {
            Self::Select => "cursor",
            Self::Fill => "fill",
        }
    }
}

pub type Cursor = (Point, Color, CursorType);
