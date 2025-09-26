use roguemetry::{Point, DIR8};
use tetra::input::Key;
use tetra::Context;

use crate::colors::Colors;
use crate::game::actions::implements::Melee;
use crate::game::World;
use crate::input;
use crate::scenes::implements::GameScene;
use crate::scenes::Transition;

use super::super::{Cursor, CursorType, GameModeImpl};

pub struct MeleeAttack {
    pub target: Option<Point>,
}

impl MeleeAttack {
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl GameModeImpl for MeleeAttack {
    fn cursors(&self, world: &World) -> Option<Vec<Cursor>> {
        Some(if let Some(target) = self.target {
            let pos = target - world.units().player().pos;
            vec![
                (pos, Colors::CURSOR_BG, CursorType::Fill),
                (pos, Colors::LIGHT_CORAL, CursorType::Select),
            ]
        } else {
            DIR8.iter()
                .copied()
                .filter(|&dir| {
                    !world
                        .map()
                        .get_tile(world.units().player().pos + dir)
                        .units
                        .is_empty()
                })
                .map(|dir| (dir.into(), Colors::RED, CursorType::Select))
                .collect()
        })
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.target = Some(game.world.borrow().units().player().pos + dir);
            game.try_rotate_player(dir);
        } else if let Some(target) = self.target {
            let action = Melee::new(target, &game.world.borrow());
            game.try_start_action(action);
            game.modes.pop();
        }
        None
    }
}
