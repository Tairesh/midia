use geometry::{Direction, DIR9};
use tetra::{input::Key, Context};

use crate::{
    colors::Colors,
    game::{actions::implements::Drop, map::TerrainInteract, World},
    input,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    Cursor, CursorType, GameModeImpl,
};

pub struct Dropping {
    selected: Option<Direction>,
}

impl Dropping {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Dropping {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Dropping {
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
                    world.map().get_tile(pos).terrain.can_stock_items()
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE, CursorType::Select))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        if world.units().player().inventory.main_hand().is_none() {
            Err("You have nothing to drop!".to_string())
        } else {
            Ok(())
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(Drop { dir }.into());
            game.modes.pop();
        }
        None
    }
}
