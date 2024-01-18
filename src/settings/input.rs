use super::Validate;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct InputSettings {
    pub repeat_interval: u32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            repeat_interval: 125,
        }
    }
}

impl Validate for InputSettings {
    fn validate(&mut self) {
        self.repeat_interval = self.repeat_interval.clamp(1, 1000);
    }
}
