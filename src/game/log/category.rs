use tetra::graphics::Color;

use crate::colors::Colors;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum LogCategory {
    #[serde(rename = "0")]
    Debug,
    #[serde(rename = "1")]
    Info,
    #[serde(rename = "2")]
    Warning,
    #[serde(rename = "3")]
    Danger,
    #[serde(rename = "4")]
    Success,
}

impl From<LogCategory> for Color {
    fn from(c: LogCategory) -> Self {
        match c {
            LogCategory::Debug => Colors::AQUA,
            LogCategory::Info => Colors::LIGHT_GRAY,
            LogCategory::Warning => Colors::ORANGE_RED,
            LogCategory::Danger => Colors::RED,
            LogCategory::Success => Colors::LIME,
        }
    }
}
