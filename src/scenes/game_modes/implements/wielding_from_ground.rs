use geometry::{Direction, DIR9};
use tetra::{input::Key, Context};

use crate::{
    colors::Colors,
    game::{actions::implements::WieldFromGround, World},
    input,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    Cursor, CursorType, GameModeImpl,
};

pub struct WieldingFromGround {
    selected: Option<Direction>,
}

impl WieldingFromGround {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for WieldingFromGround {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for WieldingFromGround {
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
                    let pos = world.player().pos + *d;
                    !world.map().get_tile(pos).items.is_empty()
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE, CursorType::Select))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        world.player().wield.can_wield(false)
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(WieldFromGround { dir }.into());
            game.modes.pop();
        }
        None
    }
}
