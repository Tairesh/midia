use roguemetry::Point;
use tetra::graphics::Color;

use crate::assets::Sprite;
use crate::game::traits::LooksLike;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorType {
    Select,
    Fill,
}

impl LooksLike for CursorType {
    fn looks_like(&self) -> Sprite {
        match self {
            Self::Select => Sprite::Cursor,
            Self::Fill => Sprite::Fill,
        }
    }
}

pub type Cursor = (Point, Color, CursorType);
