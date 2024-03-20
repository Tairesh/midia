use std::path::PathBuf;

use super::Scene;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(Scene),
    Pop,
    /// pop and push
    Replace(Scene),
    /// unload world pop all scenes except first
    GoMainMenu,
    /// load savefile and push GameScene
    LoadWorld(PathBuf),
    /// custom scene-related event
    CustomEvent(u8),
    Quit,
}
