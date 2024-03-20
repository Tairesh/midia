use geometry::Direction;
use tetra::{input::Key, Context};

use crate::{colors::Colors, game::World, input};

use super::super::{
    super::{implements::GameScene, Transition},
    Cursor, CursorType, GameModeImpl,
};

pub struct Examining {
    selected: Option<Direction>,
}

impl Examining {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Examining {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Examining {
    fn cursors(&self, _world: &World) -> Option<Vec<Cursor>> {
        self.selected.map(|selected| {
            vec![
                (selected.into(), Colors::CURSOR_BG, CursorType::Fill),
                (selected.into(), Colors::CURSOR_FG, CursorType::Select),
            ]
        })
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.examine(dir);
            game.modes.pop();
        }
        None
    }
}
