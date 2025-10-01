use rand::Rng;

use crate::game::savage::dices::DiceStack;
use crate::game::Dice;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Wound {
    Groin,
    LeftArm,
    RightArm,
    BrokenGuts,
    BatteredGuts,
    BustedGuts,
    LeftLeg,
    RightLeg,
    HideousScar,
    LeftEye,
    RightEye,
    BrainDamage,
}

impl Wound {
    pub fn name(self) -> &'static str {
        match self {
            Wound::Groin => "groin",
            Wound::LeftArm => "left arm",
            Wound::RightArm => "right arm",
            Wound::BrokenGuts => "broken guts",
            Wound::BatteredGuts => "battered guts",
            Wound::BustedGuts => "busted guts",
            Wound::LeftLeg => "left leg",
            Wound::RightLeg => "right leg",
            Wound::HideousScar => "hideous scar",
            Wound::LeftEye => "left eye",
            Wound::RightEye => "right eye",
            Wound::BrainDamage => "brain damage",
        }
    }

    pub fn random() -> Self {
        // TODO: use world's rng instead of thread_rng
        let roll = DiceStack::from((Dice::D6, Dice::D6)).roll_total();
        match roll {
            2 => Wound::Groin,
            3 => Wound::LeftArm,
            4 => Wound::RightArm,
            5..=9 => match Dice::D6.roll() {
                1..=2 => Wound::BrokenGuts,
                3..=4 => Wound::BatteredGuts,
                5..=6 => Wound::BustedGuts,
                _ => unreachable!(),
            },
            10 => {
                if rand::rng().random_bool(0.5) {
                    Wound::LeftLeg
                } else {
                    Wound::RightLeg
                }
            }
            11..=12 => match Dice::D6.roll() {
                1..=2 => Wound::HideousScar,
                3 => Wound::LeftEye,
                4 => Wound::RightEye,
                5..=6 => Wound::BrainDamage,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
