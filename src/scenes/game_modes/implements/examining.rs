use geometry::{Direction, Point};
use tetra::{graphics::Color, input::Key, Context};

use crate::{colors::Colors, game::World, input};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    GameModeImpl,
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
    fn cursors(&self, _world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            vec![]
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
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
