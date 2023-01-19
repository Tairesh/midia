use geometry::{Direction, Point, DIR9};
use tetra::{graphics::Color, input::Key, Context};

use crate::{
    colors::Colors,
    game::{actions::implements::Read, World},
    input,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    GameModeImpl,
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
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.player().pos + *d;
                    world.map().get_tile(pos).is_readable()
                })
                .map(|d| (d.into(), Colors::LIGHT_YELLOW))
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
            let action = Read::new(dir, game.world.borrow().player()).into();
            game.try_start_action(action);
            game.modes.pop();
        }
        None
    }
}
