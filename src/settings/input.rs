use std::collections::HashMap;

use tetra::input::{
    Key,
    KeyModifier::{Ctrl, Shift},
};

use crate::input::KeyWithMod;

use super::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyBindingAction {
    MainMenu,
    ClearLog,
    Examine,
    Drop,
    DropHere,
    Wear,
    PickUp,
    Reload,
    Skip,
    Observe,
    Open,
    Read,
    Close,
    RangeAttack,
    MeleeAttack,
    Throw,
    SwapHands,
    Inventory,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct InputSettings {
    pub repeat_interval: u32,
    pub keybindings: HashMap<KeyWithMod, KeyBindingAction>,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            repeat_interval: 125,
            keybindings: [
                (KeyWithMod::key(Key::Escape), KeyBindingAction::MainMenu),
                (KeyWithMod::key(Key::E), KeyBindingAction::Examine),
                (KeyWithMod::key(Key::D), KeyBindingAction::DropHere),
                (KeyWithMod::new(Key::D, Shift), KeyBindingAction::Drop),
                (KeyWithMod::key(Key::G), KeyBindingAction::PickUp),
                (KeyWithMod::key(Key::R), KeyBindingAction::Reload),
                (KeyWithMod::new(Key::C, Ctrl), KeyBindingAction::ClearLog),
                (KeyWithMod::key(Key::X), KeyBindingAction::Observe),
                (KeyWithMod::new(Key::R, Shift), KeyBindingAction::Read),
                (KeyWithMod::key(Key::O), KeyBindingAction::Open),
                (KeyWithMod::key(Key::C), KeyBindingAction::Close),
                (KeyWithMod::new(Key::A, Ctrl), KeyBindingAction::MeleeAttack),
                (KeyWithMod::new(Key::W, Shift), KeyBindingAction::Wear),
                (KeyWithMod::new(Key::X, Shift), KeyBindingAction::SwapHands),
                (KeyWithMod::key(Key::I), KeyBindingAction::Inventory),
                (KeyWithMod::key(Key::T), KeyBindingAction::Throw),
                (KeyWithMod::key(Key::F), KeyBindingAction::RangeAttack),
                (KeyWithMod::key(Key::R), KeyBindingAction::Reload),
                (KeyWithMod::key(Key::Period), KeyBindingAction::Skip),
            ]
            .into(),
        }
    }
}

impl Validate for InputSettings {
    fn validate(&mut self) {
        self.repeat_interval = self.repeat_interval.clamp(1, 1000);
    }
}
