use super::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct DebugSettings {
    pub show_fps: bool,
    // TODO: debug log, backtrace, god-mode, etc.
}

impl Validate for DebugSettings {
    fn validate(&mut self) {
        // do nothing
    }
}
