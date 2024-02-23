use super::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(default)]
pub struct DebugSettings {
    pub show_fps: bool,
    pub god_mode: bool,
    pub show_debug_log: bool,
}

impl Validate for DebugSettings {
    fn validate(&mut self) {
        // do nothing
    }
}
