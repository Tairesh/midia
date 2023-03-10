use geometry::{Point, DIR8};
use tetra::input::Key;
use tetra::Context;

use crate::colors::Colors;
use crate::game::actions::implements::Melee;
use crate::game::World;
use crate::input;
use crate::scenes::implements::GameScene;
use crate::scenes::SomeTransitions;

use super::super::{Cursor, CursorType, GameModeImpl};

pub struct ForceAttack {
    pub target: Option<Point>,
}

impl ForceAttack {
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl GameModeImpl for ForceAttack {
    fn cursors(&self, world: &World) -> Vec<Cursor> {
        if let Some(target) = self.target {
            let pos = target - world.player().pos;
            vec![
                (pos, Colors::WHITE.with_alpha(0.1), CursorType::Fill),
                (pos, Colors::LIGHT_CORAL, CursorType::Select),
            ]
        } else {
            DIR8.iter()
                .copied()
                .filter(|&dir| {
                    !world
                        .map()
                        .get_tile(world.player().pos + dir)
                        .units
                        .is_empty()
                })
                .map(|dir| (dir.into(), Colors::RED, CursorType::Select))
                .collect()
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        // TODO: select targets on distance, with two modes, as in CoQ
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.target = Some(game.world.borrow().player().pos + dir);
            game.try_rotate_player(dir);
        } else if let Some(target) = self.target {
            game.try_start_action(Melee::new(target).into());
            game.modes.pop();
        }
        None
    }
}
