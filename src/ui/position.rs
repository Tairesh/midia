#![allow(dead_code)]

use roguemetry::Vec2;

#[derive(Copy, Clone)]
pub enum Horizontal {
    ByLeft { x: f32 },
    ByCenter { x: f32 },
    ByRight { x: f32 },
    AtWindowCenterByLeft { offset: f32 },
    AtWindowCenterByCenter { offset: f32 },
    AtWindowCenterByRight { offset: f32 },
    AtWindowRightByLeft { offset: f32 },
    AtWindowRightByCenter { offset: f32 },
    AtWindowRightByRight { offset: f32 },
}

#[derive(Copy, Clone)]
pub enum Vertical {
    ByTop { y: f32 },
    ByCenter { y: f32 },
    ByBottom { y: f32 },
    AtWindowCenterByTop { offset: f32 },
    AtWindowCenterByCenter { offset: f32 },
    AtWindowCenterByBottom { offset: f32 },
    AtWindowBottomByTop { offset: f32 },
    AtWindowBottomByCenter { offset: f32 },
    AtWindowBottomByBottom { offset: f32 },
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: Horizontal,
    pub y: Vertical,
}

#[derive(Copy, Clone)]
pub enum AnchorX {
    Left,
    Center,
    Right,
}

impl AnchorX {
    pub fn to_position(self, x: f32) -> Horizontal {
        match self {
            AnchorX::Left => Horizontal::ByLeft { x },
            AnchorX::Center => Horizontal::ByCenter { x },
            AnchorX::Right => Horizontal::ByRight { x },
        }
    }
}

#[derive(Copy, Clone)]
pub enum AnchorY {
    Top,
    Center,
    Bottom,
}

impl AnchorY {
    pub fn to_position(self, y: f32) -> Vertical {
        match self {
            AnchorY::Top => Vertical::ByTop { y },
            AnchorY::Center => Vertical::ByCenter { y },
            AnchorY::Bottom => Vertical::ByBottom { y },
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, anchor_x: AnchorX, anchor_y: AnchorY) -> Position {
        Position {
            x: anchor_x.to_position(x),
            y: anchor_y.to_position(y),
        }
    }

    pub fn by_left_top(x: f32, y: f32) -> Position {
        Position::new(x, y, AnchorX::Left, AnchorY::Top)
    }

    pub fn by_right_top(x_offset: f32, y: f32) -> Position {
        Position {
            x: Horizontal::AtWindowRightByRight { offset: x_offset },
            y: AnchorY::Top.to_position(y),
        }
    }

    pub fn at_center_by_left_top(x_offset: f32, y_offset: f32) -> Position {
        Position {
            x: Horizontal::AtWindowCenterByLeft { offset: x_offset },
            y: Vertical::AtWindowCenterByTop { offset: y_offset },
        }
    }

    pub fn at_center_by_right_top(x_offset: f32, y_offset: f32) -> Position {
        Position {
            x: Horizontal::AtWindowCenterByRight { offset: x_offset },
            y: Vertical::AtWindowCenterByTop { offset: y_offset },
        }
    }

    pub fn at_center_by_left_bottom(x_offset: f32, y_offset: f32) -> Position {
        Position {
            x: Horizontal::AtWindowCenterByLeft { offset: x_offset },
            y: Vertical::AtWindowCenterByBottom { offset: y_offset },
        }
    }

    pub fn at_center_by_right_bottom(x_offset: f32, y_offset: f32) -> Position {
        Position {
            x: Horizontal::AtWindowCenterByRight { offset: x_offset },
            y: Vertical::AtWindowCenterByBottom { offset: y_offset },
        }
    }

    pub fn center() -> Position {
        Position {
            x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
            y: Vertical::AtWindowCenterByCenter { offset: 0.0 },
        }
    }

    pub fn horizontal_center(offset: f32, y: Vertical) -> Position {
        Position {
            x: Horizontal::AtWindowCenterByCenter { offset },
            y,
        }
    }

    #[allow(dead_code)]
    pub fn vertical_center(offset: f32, x: Horizontal) -> Position {
        Position {
            x,
            y: Vertical::AtWindowCenterByCenter { offset },
        }
    }

    pub fn as_vec(&self, owner_size: Vec2, window_size: (i32, i32)) -> Vec2 {
        let x = match self.x {
            Horizontal::ByLeft { x } => x,
            Horizontal::ByCenter { x } => x - owner_size.x / 2.0,
            Horizontal::ByRight { x } => x - owner_size.x,
            Horizontal::AtWindowCenterByCenter { offset } => {
                (window_size.0 / 2) as f32 - (owner_size.x / 2.0) + offset
            }
            Horizontal::AtWindowCenterByLeft { offset } => (window_size.0 / 2) as f32 + offset,
            Horizontal::AtWindowCenterByRight { offset } => {
                (window_size.0 / 2) as f32 - owner_size.x + offset
            }
            Horizontal::AtWindowRightByLeft { offset } => window_size.0 as f32 + offset,
            Horizontal::AtWindowRightByCenter { offset } => {
                window_size.0 as f32 - (owner_size.x / 2.0) + offset
            }
            Horizontal::AtWindowRightByRight { offset } => {
                window_size.0 as f32 - owner_size.x + offset
            }
        };
        let y = match self.y {
            Vertical::ByTop { y } => y,
            Vertical::ByCenter { y } => y - owner_size.y / 2.0,
            Vertical::ByBottom { y } => y - owner_size.y,
            Vertical::AtWindowCenterByCenter { offset } => {
                (window_size.1 / 2) as f32 - (owner_size.y / 2.0) + offset
            }
            Vertical::AtWindowCenterByTop { offset } => (window_size.1 / 2) as f32 + offset,
            Vertical::AtWindowCenterByBottom { offset } => {
                (window_size.1 / 2) as f32 - owner_size.y + offset
            }
            Vertical::AtWindowBottomByTop { offset } => window_size.1 as f32 + offset,
            Vertical::AtWindowBottomByCenter { offset } => {
                window_size.1 as f32 - (owner_size.y / 2.0) + offset
            }
            Vertical::AtWindowBottomByBottom { offset } => {
                window_size.1 as f32 - owner_size.y + offset
            }
        };
        Vec2::new(x.round(), y.round())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use roguemetry::Vec2;

    use super::{AnchorX, AnchorY, Horizontal, Position, Vertical};

    #[test_case(Position::new(0.0, 0.0, AnchorX::Left, AnchorY::Top), Vec2::zero())]
    #[test_case(Position::new(300.0, 400.0, AnchorX::Right, AnchorY::Bottom), Vec2::new(200.0, 200.0))]
    #[test_case(Position::new(300.0, 300.0, AnchorX::Center, AnchorY::Center), Vec2::new(250.0, 200.0))]
    #[test_case(Position::center(), Vec2::new(350.0, 200.0))]
    #[test_case(Position::horizontal_center(10.0, Vertical::AtWindowCenterByCenter { offset: 10.0 }), Vec2::new(360.0, 210.0))]
    #[test_case(Position::vertical_center(10.0, Horizontal::AtWindowCenterByCenter { offset: 10.0 }), Vec2::new(360.0, 210.0))]
    #[test_case(Position { x: Horizontal::AtWindowRightByRight { offset: -10.0 }, y: Vertical::AtWindowBottomByBottom { offset: -10.0 } }, Vec2::new(690.0, 390.0))]
    fn test_position_to_vec(pos: Position, expected: Vec2) {
        let owner_size = Vec2::new(100.0, 200.0);
        let window_size = (800, 600);
        assert_eq!(pos.as_vec(owner_size, window_size), expected);
    }
}
