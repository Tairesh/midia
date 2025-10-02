use std::path::PathBuf;

use super::SceneKind;

#[derive(Debug, Clone, Default)]
pub enum Transition {
    /// do nothing
    #[default]
    None,
    /// push scene on top of stack
    Push(SceneKind),
    /// pop current scene
    Pop,
    /// pop and push
    Switch(SceneKind),
    /// unload world, pop all scenes except first
    ExitToMainMenu,
    /// load savefile and push `GameScene`
    LoadWorld(PathBuf),
    /// custom scene-related event
    CustomEvent(u8),
    Quit,
}

impl Transition {
    pub fn is_none(&self) -> bool {
        matches!(self, Transition::None)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}
