use enum_iterator::{next_cycle, previous_cycle, Sequence};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Sequence, Debug, Copy, Clone, Eq, PartialEq)]
pub enum MainHand {
    #[serde(rename = "r")]
    Right,
    #[serde(rename = "l")]
    Left,
    #[serde(rename = "a")]
    Ambidexter,
}

impl MainHand {
    pub fn name(self) -> &'static str {
        self.into()
    }

    pub fn next(self) -> Self {
        next_cycle(&self).unwrap()
    }

    pub fn prev(self) -> Self {
        previous_cycle(&self).unwrap()
    }
}

impl From<MainHand> for &str {
    fn from(s: MainHand) -> Self {
        match s {
            MainHand::Right => "Right",
            MainHand::Left => "Left",
            MainHand::Ambidexter => "Ambidexter",
        }
    }
}

impl Distribution<MainHand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MainHand {
        if rng.gen_bool(0.01) {
            MainHand::Ambidexter
        } else if rng.gen_bool(0.16) {
            MainHand::Left
        } else {
            MainHand::Right
        }
    }
}
