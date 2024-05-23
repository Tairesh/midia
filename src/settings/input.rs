use std::collections::HashMap;

use tetra::input::Key;

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
                (Key::Escape.into(), KeyBindingAction::MainMenu),
                (Key::E.into(), KeyBindingAction::Examine),
                (Key::D.into(), KeyBindingAction::DropHere),
                (KeyWithMod::shift(Key::D), KeyBindingAction::Drop),
                (Key::G.into(), KeyBindingAction::PickUp),
                (Key::R.into(), KeyBindingAction::Reload),
                (KeyWithMod::ctrl(Key::C), KeyBindingAction::ClearLog),
                (Key::X.into(), KeyBindingAction::Observe),
                (KeyWithMod::shift(Key::R), KeyBindingAction::Read),
                (Key::O.into(), KeyBindingAction::Open),
                (Key::C.into(), KeyBindingAction::Close),
                (KeyWithMod::ctrl(Key::A), KeyBindingAction::MeleeAttack),
                (KeyWithMod::shift(Key::W), KeyBindingAction::Wear),
                (KeyWithMod::shift(Key::X), KeyBindingAction::SwapHands),
                (Key::I.into(), KeyBindingAction::Inventory),
                (Key::T.into(), KeyBindingAction::Throw),
                (Key::F.into(), KeyBindingAction::RangeAttack),
                (Key::R.into(), KeyBindingAction::Reload),
                (Key::Period.into(), KeyBindingAction::Skip),
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
