use roguemetry::Vec2;

/// Specifies how to position a UI element horizontally relative to the window.
#[derive(Copy, Clone)]
pub enum Horizontal {
    /// Align left edge of the element to the left edge of the window
    LeftByLeft,
    /// Align left edge of the element to the horizontal center of the window
    CenterByLeft,
    /// Align center of the element to the horizontal center of the window
    CenterByCenter,
    /// Align right edge of the element to the horizontal center of the window
    CenterByRight,
    /// Align right edge of the element to the right edge of the window
    RightByRight,
}

/// Specifies how to position a UI element vertically relative to the window.
#[derive(Copy, Clone)]
pub enum Vertical {
    /// Align top edge of the element to the top edge of the window
    TopByTop,
    /// Align center of the element to the top edge of the window
    TopByCenter,
    /// Align bottom edge of the element to the top edge of the window
    TopByBottom,
    /// Align top edge of the element to the vertical center of the window
    CenterByTop,
    /// Align center of the element to the vertical center of the window
    CenterByCenter,
    /// Align bottom edge of the element to the vertical center of the window
    CenterByBottom,
    /// Align bottom edge of the element to the bottom edge of the window
    BottomByBottom,
}

#[derive(Copy, Clone)]
pub struct Position {
    horizontal: Horizontal,
    vertical: Vertical,
    offset: Vec2,
}

impl Position {
    pub fn new(horizontal: Horizontal, vertical: Vertical, offset: Vec2) -> Position {
        Position {
            horizontal,
            vertical,
            offset,
        }
    }

    pub fn zero() -> Position {
        Position {
            horizontal: Horizontal::LeftByLeft,
            vertical: Vertical::TopByTop,
            offset: Vec2::zero(),
        }
    }

    pub fn by_left_top(offset: Vec2) -> Position {
        Position::new(Horizontal::LeftByLeft, Vertical::TopByTop, offset)
    }

    pub fn by_right_top(offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::RightByRight,
            vertical: Vertical::TopByTop,
            offset,
        }
    }

    pub fn at_center_by_left_top(offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::CenterByLeft,
            vertical: Vertical::CenterByTop,
            offset,
        }
    }

    pub fn at_center_by_right_top(offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::CenterByRight,
            vertical: Vertical::CenterByTop,
            offset,
        }
    }

    pub fn at_center_by_left_bottom(offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::CenterByLeft,
            vertical: Vertical::CenterByBottom,
            offset,
        }
    }

    pub fn at_center_by_right_bottom(offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::CenterByRight,
            vertical: Vertical::CenterByBottom,
            offset,
        }
    }

    pub fn center() -> Position {
        Position {
            horizontal: Horizontal::CenterByCenter,
            vertical: Vertical::CenterByCenter,
            offset: Vec2::zero(),
        }
    }

    pub fn horizontal_center(vertical: Vertical, offset: Vec2) -> Position {
        Position {
            horizontal: Horizontal::CenterByCenter,
            vertical,
            offset,
        }
    }

    pub fn calc(&self, owner_size: Vec2, window_size: (i32, i32)) -> Vec2 {
        let window_size = Vec2::new(window_size.0 as f32, window_size.1 as f32);
        let x = match self.horizontal {
            Horizontal::LeftByLeft => 0.0,
            Horizontal::CenterByCenter => window_size.x / 2.0 - owner_size.x / 2.0,
            Horizontal::CenterByLeft => window_size.x / 2.0,
            Horizontal::CenterByRight => window_size.x / 2.0 - owner_size.x,
            Horizontal::RightByRight => window_size.x - owner_size.x,
        };
        let y = match self.vertical {
            Vertical::TopByTop => 0.0,
            Vertical::TopByCenter => -owner_size.y / 2.0,
            Vertical::TopByBottom => -owner_size.y,
            Vertical::CenterByCenter => window_size.y / 2.0 - owner_size.y / 2.0,
            Vertical::CenterByTop => window_size.y / 2.0,
            Vertical::CenterByBottom => window_size.y / 2.0 - owner_size.y,
            Vertical::BottomByBottom => window_size.y - owner_size.y,
        };

        (Vec2::new(x, y) + self.offset).round()
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::Vec2;
    use test_case::test_case;

    use super::{Horizontal, Position, Vertical};

    #[test_case(Position::by_left_top(Vec2::new(0.0, 0.0)), Vec2::zero(); "left_top")]
    #[test_case(Position::by_left_top(Vec2::new(10.0, 20.0)), Vec2::new(10.0, 20.0); "left_top_offset")]
    #[test_case(Position::by_right_top(Vec2::new(-10.0, 20.0)), Vec2::new(690.0, 20.0); "right_top_offset")]
    #[test_case(Position::center(), Vec2::new(350.0, 200.0); "center")]
    #[test_case(
        Position::horizontal_center(Vertical::TopByTop, Vec2::new(0.0, 10.0)),
        Vec2::new(350.0, 10.0);
        "horizontal_center_top"
    )]
    #[test_case(Position::new(Horizontal::RightByRight, Vertical::BottomByBottom, Vec2::new(-10.0, -10.0)), Vec2::new(690.0, 390.0); "right_bottom_offset")]
    #[test_case(Position::at_center_by_left_top(Vec2::new(50.0, 30.0)), Vec2::new(450.0, 330.0); "center_by_left_top")]
    #[test_case(Position::at_center_by_right_top(Vec2::new(-50.0, 30.0)), Vec2::new(250.0, 330.0); "center_by_right_top")]
    #[test_case(Position::at_center_by_left_bottom(Vec2::new(50.0, -30.0)), Vec2::new(450.0, 70.0); "center_by_left_bottom")]
    #[test_case(Position::at_center_by_right_bottom(Vec2::new(-50.0, -30.0)), Vec2::new(250.0, 70.0); "center_by_right_bottom")]
    fn test_position_calc(pos: Position, expected: Vec2) {
        let owner_size = Vec2::new(100.0, 200.0);
        let window_size = (800, 600);
        assert_eq!(pos.calc(owner_size, window_size), expected);
    }
}
