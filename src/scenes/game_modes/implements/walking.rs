use std::time::Instant;

use roguemetry::Direction;
use tetra::{input::KeyModifier, Context};

use crate::{
    colors::Colors,
    game::{
        actions::implements::{DropMainHand, Reload, Skip, Walk, Wear},
        traits::Name,
        BodySlot, Fighter, LogEvent,
    },
    input,
    settings::{KeyBindingAction, Settings},
};

use super::super::{
    super::{implements::GameScene, SceneKind, Transition},
    implements::{Interacting, MeleeAttack, Observing, PikeAttack, Shooting, Throwing},
    GameModeImpl, PlayerCommand,
};

pub struct Walking {
    last_walk: Instant,
}

impl Walking {
    pub fn new() -> Self {
        Self {
            last_walk: Instant::now(),
        }
    }
}

impl Default for Walking {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Walking {
    // TODO: refactor this method
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Transition {
        for key in input::get_key_with_mod_pressed(ctx) {
            if let Some(&action) = Settings::instance().input.keybindings.get(&key) {
                match action {
                    KeyBindingAction::MainMenu => {
                        return Transition::Push(SceneKind::GameMenu);
                    }
                    KeyBindingAction::Examine => {
                        game.push_mode(Interacting::new(PlayerCommand::Examine).into());
                    }
                    KeyBindingAction::DropHere => {
                        game.try_start_action(
                            DropMainHand {
                                dir: Direction::Here,
                            }
                            .into(),
                        );
                    }
                    KeyBindingAction::Drop => {
                        game.push_mode(Interacting::new(PlayerCommand::Drop).into());
                    }
                    KeyBindingAction::PickUp => {
                        game.push_mode(Interacting::new(PlayerCommand::WieldFromGround).into());
                    }
                    KeyBindingAction::ClearLog => {
                        game.log.clear();
                    }
                    KeyBindingAction::Observe => {
                        game.push_mode(Observing::new().into());
                    }
                    KeyBindingAction::Open => {
                        game.push_mode(Interacting::new(PlayerCommand::Open).into());
                    }
                    KeyBindingAction::Read => {
                        game.push_mode(Interacting::new(PlayerCommand::Read).into());
                    }
                    KeyBindingAction::Close => {
                        game.push_mode(Interacting::new(PlayerCommand::Close).into());
                    }
                    KeyBindingAction::Wear => {
                        game.try_start_action(Wear {}.into());
                    }
                    KeyBindingAction::MeleeAttack => {
                        game.push_mode(MeleeAttack::new().into());
                    }
                    KeyBindingAction::Throw => {
                        game.push_mode(Throwing::new().into());
                    }
                    KeyBindingAction::RangeAttack => {
                        let units = game.world.units();
                        if let Some(weapon) = units.player().inventory.main_hand() {
                            if weapon.melee_damage().distance > 0 {
                                drop(units);
                                game.push_mode(PikeAttack::new().into());
                                return Transition::None;
                            }
                        }
                        drop(units);
                        game.push_mode(Shooting::new().into());
                    }
                    KeyBindingAction::Reload => {
                        game.try_start_action(Reload::new());
                    }
                    KeyBindingAction::SwapHands => {
                        game.world.units_mut().player_mut().inventory.swap_hands();
                        let event =
                            LogEvent::info("You swap your hands", game.world.units().player().pos);
                        game.world.log().push(event);
                        game.update_ui(ctx);
                    }
                    KeyBindingAction::Inventory => {
                        // TODO: inventory game scene
                        let items: Vec<String> = game
                            .world
                            .units()
                            .player()
                            .inventory
                            .iter_wear()
                            .map(|i| i.name().to_string())
                            .collect();
                        let armor = game.world.units().player().armor(BodySlot::Torso);
                        let toughness = game
                            .world
                            .units()
                            .player()
                            .personality
                            .char_sheet
                            .toughness();
                        let parry = game.world.units().player().parry();
                        game.log.log(
                            format!(
                                "You wear: {}, armor value is {armor}, toughness: {toughness}, parry: {parry}",
                                items.join(", ")
                            ),
                            Colors::WHITE_SMOKE,
                        );
                    }
                    KeyBindingAction::Skip => {
                        game.try_start_action(Skip::one());
                    }
                }
            }
        }

        if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_walk).subsec_millis()
                > Settings::instance().input.repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                if dir.is_here() {
                    game.try_start_action(Skip::one());
                } else {
                    game.try_rotate_player(dir);
                    game.try_start_action(Walk::new(dir));
                }
            }
        }

        Transition::None
    }
}
