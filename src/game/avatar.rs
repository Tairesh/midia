#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use crate::game::bodies::OrganData;

use super::{
    ai::ZombieAI,
    bodies::{Body, Freshness},
    map::items::{Cloak, Hat},
    races::{helpers::body, Personality},
    Action, Item,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum Soul {
    Player(Personality),
    Zombie(Personality, ZombieAI),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub body: Body,
    pub soul: Soul,
    pub pos: Point,
    pub action: Option<Action>,
    // TODO: rotation of multitile body
    pub vision: TwoDimDirection,
    // TODO: custom struct with hands counter
    pub wield: Vec<Item>,
    pub stamina: u8,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn player(personality: Personality, pos: Point) -> Self {
        let mut body = body(OrganData::new(&personality, Freshness::Fresh));
        body.wear.push(Cloak::new().into());
        body.wear.push(Hat::new().into());
        Self::new(body, Soul::Player(personality), pos)
    }

    pub fn zombie(personality: Personality, body: Body, pos: Point) -> Self {
        Self::new(body, Soul::Zombie(personality, ZombieAI::default()), pos)
    }

    pub fn new(body: Body, soul: Soul, pos: Point) -> Self {
        Avatar {
            body,
            soul,
            pos,
            action: None,
            vision: TwoDimDirection::East,
            wield: Vec::new(),
            stamina: 100,
        }
    }

    pub fn person(&self) -> &Personality {
        match &self.soul {
            Soul::Player(p) | Soul::Zombie(p, ..) => p,
        }
    }

    pub fn name_for_actions(&self) -> String {
        match &self.soul {
            Soul::Player(..) => "You".to_string(),
            Soul::Zombie(person, ..) => format!("Zombie {}", person.mind.name),
        }
    }
}
