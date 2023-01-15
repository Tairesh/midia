use geometry::{Direction, Point, DIR9};
use tetra::{graphics::Color, input::Key, Context};

use crate::{
    colors::Colors,
    game::{actions::implements::Wield, World},
    input,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    GameModeImpl,
};

pub struct Wielding {
    selected: Option<Direction>,
}

impl Wielding {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Wielding {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Wielding {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.player().pos + *d;
                    !world.map().get_tile(pos).items.is_empty()
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        if world.player().wield.has_free_space() {
            Ok(())
        } else {
            Err(format!(
                "You already have {} in hands",
                world.player().wield.names()
            ))
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(Wield { dir }.into());
            game.modes.pop();
        }
        None
    }
}
