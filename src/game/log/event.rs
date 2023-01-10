use geometry::Point;

use super::LogCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogEvent {
    #[serde(rename = "m")]
    pub msg: String,
    #[serde(rename = "p")]
    pub pos: Point,
    #[serde(rename = "c")]
    pub category: LogCategory,
}

impl LogEvent {
    pub fn new<S: Into<String>>(msg: S, pos: Point, category: LogCategory) -> Self {
        Self {
            msg: msg.into(),
            pos,
            category,
        }
    }
}
