use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BodySlot {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

impl BodySlot {
    pub fn name(self) -> &'static str {
        match self {
            BodySlot::Head => "head",
            BodySlot::Torso => "torso",
            BodySlot::LeftArm => "left arm",
            BodySlot::RightArm => "right arm",
            BodySlot::LeftLeg => "left leg",
            BodySlot::RightLeg => "right leg",
        }
    }

    pub fn iterator() -> impl Iterator<Item = BodySlot> {
        [
            BodySlot::Head,
            BodySlot::Torso,
            BodySlot::LeftArm,
            BodySlot::RightArm,
            BodySlot::LeftLeg,
            BodySlot::RightLeg,
        ]
        .into_iter()
    }
}

impl Distribution<BodySlot> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BodySlot {
        unsafe { std::mem::transmute(rng.gen::<u8>() % BodySlot::iterator().count() as u8) }
    }
}
