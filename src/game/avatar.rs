#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::{
    map::items::{Cloak, Hat},
    races::Personality,
    savage::CharSheet,
    Action, Item,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub personality: Personality,
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter and methods to return names and icons for UI
    pub wield: Vec<Item>,
    // TODO: custom struct with layers for dress and methods to return names and icons for UI
    pub wear: Vec<Item>,
    pub char_sheet: CharSheet,
    // TODO: stamina
    // TODO: traits
}

impl Avatar {
    pub fn new(personality: Personality, char_sheet: CharSheet, pos: Point) -> Self {
        Avatar {
            personality,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            wear: Vec::new(),
            char_sheet,
        }
    }

    // TODO: remove this and select dress in create character scene
    pub fn dressed_default(personality: Personality, char_sheet: CharSheet, pos: Point) -> Self {
        Self {
            wear: vec![Hat::new().into(), Cloak::new().into()],
            ..Self::new(personality, char_sheet, pos)
        }
    }

    pub fn name_for_actions(&self) -> String {
        if self.personality.is_player {
            "You".to_string()
        } else {
            self.personality.mind.name.clone()
        }
    }
}
