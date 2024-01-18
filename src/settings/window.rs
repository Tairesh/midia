use super::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WindowSettings {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 1366,
            height: 768,
            fullscreen: false,
        }
    }
}

impl Validate for WindowSettings {
    fn validate(&mut self) {
        self.width = self.width.clamp(1366, 1920);
        self.height = self.height.clamp(768, 1280);
    }
}
