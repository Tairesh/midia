use roguemetry::{Direction, DIR9};
use tetra::input::Key;
use tetra::Context;

use crate::colors::Colors;
use crate::game::actions::implements::{Close, DropMainHand, Open, Read, WieldFromGround};
use crate::game::World;
use crate::input;
use crate::scenes::game_modes::{Cursor, CursorType};

use super::super::{
    super::{implements::GameScene, Transition},
    GameModeImpl, PlayerCommand,
};

pub struct Interacting {
    command: PlayerCommand,
    selected: Option<Direction>,
}

impl Interacting {
    pub fn new(command: PlayerCommand) -> Self {
        Self {
            command,
            selected: None,
        }
    }
}

impl GameModeImpl for Interacting {
    fn cursors(&self, world: &World) -> Option<Vec<Cursor>> {
        Some(if let Some(selected) = self.selected {
            vec![
                (selected.into(), Colors::CURSOR_BG, CursorType::Fill),
                (selected.into(), Colors::CURSOR_FG, CursorType::Select),
            ]
        } else {
            DIR9.into_iter()
                .filter(|&d| {
                    let pos = world.units().player().pos + d;
                    self.command.highlight_tile(world.map().get_tile(pos))
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE, CursorType::Select))
                .collect()
        })
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        self.command.can_start(world)
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            if self.command == PlayerCommand::Examine {
                game.examine(dir);
            } else {
                let action = match self.command {
                    PlayerCommand::Open => Open { dir }.into(),
                    PlayerCommand::Close => Close { dir }.into(),
                    PlayerCommand::Read => Read::new(dir, game.world.borrow().units().player()),
                    PlayerCommand::Drop => DropMainHand { dir }.into(),
                    PlayerCommand::WieldFromGround => WieldFromGround { dir }.into(),
                    PlayerCommand::Examine => unreachable!(),
                };
                game.try_start_action(action);
            }
            game.modes.pop();
        }
        None
    }
}
