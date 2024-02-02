use geometry::{Direction, DIR9};
use tetra::{input::Key, Context};

use crate::{
    colors::Colors,
    game::{actions::implements::Read, World},
    input,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    Cursor, CursorType, GameModeImpl,
};

pub struct Reading {
    selected: Option<Direction>,
}

impl Reading {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Reading {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Reading {
    fn cursors(&self, world: &World) -> Vec<Cursor> {
        if let Some(selected) = self.selected {
            vec![
                (
                    selected.into(),
                    Colors::WHITE.with_alpha(0.1),
                    CursorType::Fill,
                ),
                (selected.into(), Colors::LIME, CursorType::Select),
            ]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.units().player().pos + *d;
                    world.map().get_tile(pos).is_readable()
                })
                .map(|d| (d.into(), Colors::LIGHT_YELLOW, CursorType::Select))
                .collect()
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            let action = Read::new(dir, game.world.borrow().units().player()).into();
            game.try_start_action(action);
            game.modes.pop();
        }
        None
    }
}
