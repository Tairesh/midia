use std::path::PathBuf;

use super::Scene;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(Scene),
    Pop,
    GoMainMenu,     // unload world and pop to first scene
    Replace(Scene), // pop and push
    LoadWorld(PathBuf),
    CustomEvent(u8),
    Quit,
}

pub type SomeTransitions = Option<Vec<Transition>>;
