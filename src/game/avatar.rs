#![allow(dead_code)]

use geometry::{Point, TwoDimDirection};

use super::{
    ai::ZombieAI,
    bodies::{Body, Freshness},
    human::{helpers::human_body, Personality},
    map::items::{Cloak, Hat},
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
    pub pos: Point,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
    // TODO: rotation of multitile body
    pub wield: Vec<Item>,
    // TODO: custom struct with hands counter
    pub stamina: u8,
    pub soul: Soul,
    // TODO: traits
    // TODO: skills
}

impl Avatar {
    pub fn player(personality: Personality, pos: Point) -> Self {
        let mut body = human_body(&personality, Freshness::Fresh);
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
