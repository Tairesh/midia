#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::{
    ai::{Brain, ZombieAI},
    bodies::{Body, Freshness, OrganData},
    map::items::{Cloak, Hat},
    races::{helpers::body, Personality},
    Action, Item,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Avatar {
    pub body: Body,
    pub personality: Personality,
    #[serde(skip)] // TODO: serialize brains
    pub ai: Option<Box<dyn Brain>>,
    pub pos: Point,
    pub action: Option<Action>,
    // TODO: rotation of multitile body
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter and methods to return names and icons for UI
    pub wield: Vec<Item>,
    pub stamina: u8,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn new(
        body: Body,
        personality: Personality,
        ai: Option<Box<dyn Brain>>,
        pos: Point,
    ) -> Self {
        Avatar {
            body,
            personality,
            ai,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            stamina: 100,
        }
    }

    pub fn player(personality: Personality, pos: Point) -> Self {
        let mut body = body(OrganData::new(&personality, Freshness::Fresh));
        body.wear.push(Cloak::new().into());
        body.wear.push(Hat::new().into());
        Self::new(body, personality, None, pos)
    }

    pub fn zombie(personality: Personality, body: Body, pos: Point) -> Self {
        Self::new(body, personality, Some(Box::new(ZombieAI::new())), pos)
    }

    pub fn name_for_actions(&self) -> String {
        if self.ai.is_none() {
            "You".to_string()
        } else {
            self.personality.mind.name.clone()
        }
    }
}
