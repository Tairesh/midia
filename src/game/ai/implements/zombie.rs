use geometry::Direction;
use rand::{thread_rng, Rng};

use super::super::super::actions::{
    implements::{Skip, Walk},
    ActionType,
};
use super::super::Brain;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ZombieAI {
    action: ActionType,
}

impl ZombieAI {
    pub fn new() -> Self {
        Self {
            action: Skip {}.into(),
        }
    }
}

impl Default for ZombieAI {
    fn default() -> Self {
        Self::new()
    }
}

impl Brain for ZombieAI {
    fn plan(&mut self) {
        // TODO: use world.rng
        let mut rng = thread_rng();

        self.action = Walk {
            dir: match rng.gen_range(0..5) {
                0 => Direction::East,
                1 => Direction::West,
                2 => Direction::North,
                3 => Direction::South,
                4 => Direction::Here,
                _ => unreachable!(),
            },
        }
        .into();
    }

    fn action(&self) -> Option<ActionType> {
        Some(self.action)
    }
}
