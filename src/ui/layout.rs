use crate::ui::Position;
use roguemetry::{Rect, Vec2};
use tetra::Context;

/// Layout information for a UI element
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// Position of the element relative to window
    position: Position,
    /// Calculated rectangle of the element on the screen
    rect: Option<Rect>,
}

impl Layout {
    /// Creates a new Layout with the given position and no calculated rectangle
    pub fn new(position: Position) -> Self {
        Self {
            position,
            rect: None,
        }
    }

    /// Returns the position of the layout
    pub fn position(&self) -> Position {
        self.position
    }

    /// Sets the position of the layout
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    /// Returns the calculated rectangle of the layout, if any
    pub fn rect(&self) -> Rect {
        self.rect.expect("Rect of not initialized")
    }

    /// Sets the calculated rectangle of the layout
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }

    /// Updates the calculated rectangle based on the owner size and window size
    pub fn update(&mut self, owner_size: Vec2, window_size: (i32, i32)) {
        let left_top = self.position().calc(owner_size, window_size);
        self.set_rect(Rect::new(
            left_top.x,
            left_top.y,
            owner_size.x,
            owner_size.y,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_update() {
        let mut layout = Layout::new(Position::center());
        let owner_size = Vec2::new(100.0, 50.0);
        let window_size = (800, 600);

        layout.update(owner_size, window_size);
        let rect = layout.rect();

        assert_eq!(rect.x, 350.0);
        assert_eq!(rect.y, 275.0);
        assert_eq!(rect.w, 100.0);
        assert_eq!(rect.h, 50.0);
    }
}
