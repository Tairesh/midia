#![allow(dead_code)]

use geometry::Point;

use crate::lang::Capitalize;

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
    pub fn new(msg: impl Into<String>, pos: Point, category: LogCategory) -> Self {
        Self {
            msg: msg.into().capitalize(),
            pos,
            category,
        }
    }

    pub fn debug(msg: impl Into<String>, pos: Point) -> Self {
        Self::new(msg, pos, LogCategory::Debug)
    }

    pub fn info(msg: impl Into<String>, pos: Point) -> Self {
        Self::new(msg, pos, LogCategory::Info)
    }

    pub fn warning(msg: impl Into<String>, pos: Point) -> Self {
        Self::new(msg, pos, LogCategory::Warning)
    }

    pub fn danger(msg: impl Into<String>, pos: Point) -> Self {
        Self::new(msg, pos, LogCategory::Danger)
    }

    pub fn success(msg: impl Into<String>, pos: Point) -> Self {
        Self::new(msg, pos, LogCategory::Success)
    }
}
