use geometry::{Direction, Point, DIR9};
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use crate::{
    colors::Colors,
    game::{actions::implements::Close, map::TerrainInteract, World},
    input,
    scenes::{implements::GameScene, SomeTransitions},
};

use super::super::GameModeImpl;

pub struct Closing {
    selected: Option<Direction>,
}

impl Closing {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Closing {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Closing {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.into_iter()
                .filter(|&d| {
                    let pos = world.player().pos + d;
                    world.map().get_tile(pos).terrain.can_be_closed()
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE))
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
            game.try_start_action(Close { dir }.into());
            game.modes.pop();
        }
        None
    }
}
