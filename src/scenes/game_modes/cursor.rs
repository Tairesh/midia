use geometry::Point;
use tetra::graphics::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorType {
    Select,
    Fill,
}

impl CursorType {
    pub fn looks_like(self) -> &'static str {
        match self {
            Self::Select => "cursor",
            Self::Fill => "fill",
        }
    }
}

pub type Cursor = (Point, Color, CursorType);
