use crate::game::races::Race;
use crate::game::{Dice, HitResult};

use super::{Attribute, Attributes, DiceWithModifier, Skill, Skills, Wound};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CharSheet {
    #[serde(default)]
    pub wild_card: bool,
    pub race: Race,
    pub age: u8,
    pub attributes: Attributes,
    pub skills: Skills,
    pub wounds: Vec<Wound>,
    #[serde(default)]
    pub shock: bool,
    #[serde(default)]
    pub last_shock_out_roll: u128,
    // TODO: add death checks
}

impl CharSheet {
    pub fn new(
        wild_card: bool,
        race: Race,
        age: u8,
        attributes: Attributes,
        skills: Skills,
    ) -> Self {
        Self {
            wild_card,
            race,
            age,
            attributes,
            skills,
            wounds: vec![],
            shock: false,
            last_shock_out_roll: 0,
        }
    }

    pub fn default(wild_card: bool, race: Race, age: u8) -> Self {
        let attributes = Attributes::default();
        let skills = Skills::default(race);
        Self::new(wild_card, race, age, attributes, skills)
    }

    pub fn random(wild_card: bool, race: Race, age: u8) -> Self {
        let attributes = Attributes::random();
        let skills = Skills::random(&attributes, race);
        Self::new(wild_card, race, age, attributes, skills)
    }

    pub fn calc_skill_points(&self) -> i8 {
        self.skills.calc_skill_points(&self.attributes, self.race)
    }

    pub fn parry(&self) -> u8 {
        2 + self.get_skill_with_modifiers(Skill::Fighting).value() / 2
    }

    pub fn toughness(&self) -> u8 {
        (2 + self.get_attribute_with_modifiers(Attribute::Vigor).value() as i8 / 2
            + self.race.toughness_bonus())
        .max(0) as u8
    }

    pub fn walk_koeff(&self) -> f32 {
        // TODO: write tests for moving for different races and ages
        let k_age = match self.age {
            0 => 100.0,
            1..=3 => 10.0,
            4..=10 => 3.0,
            11.. => 1.0,
        };
        let mut k_wounds = 1.0;
        for &wound in &self.wounds {
            if wound == Wound::LeftLeg || wound == Wound::RightLeg {
                k_wounds *= 0.75;
            }
        }
        let k_shock = if self.shock { 0.5 } else { 1.0 };

        k_age * k_wounds * k_shock * self.race.walk_koeff()
    }

    pub fn get_attribute_with_modifiers(&self, attribute: Attribute) -> DiceWithModifier {
        let mut value = self.attributes.get_attribute(attribute);
        match attribute {
            Attribute::Vigor => {
                if self.wounds.contains(&Wound::BatteredGuts) {
                    value -= 1;
                }
            }
            Attribute::Agility => {
                if self.wounds.contains(&Wound::BrokenGuts) {
                    value -= 1;
                }
            }
            Attribute::Strength => {
                if self.wounds.contains(&Wound::BustedGuts) {
                    value -= 1;
                }
            }
            Attribute::Smarts => {
                if self.wounds.contains(&Wound::BrainDamage) {
                    value -= 1;
                }
            }
            Attribute::Spirit => {}
        }
        value -= self.wounds.len() as i8;
        value.into()
    }

    pub fn get_skill_with_modifiers(&self, skill: Skill) -> DiceWithModifier {
        DiceWithModifier::from(self.skills.get_skill(skill))
            .with_modifier(-(self.wounds.len() as i8))
    }

    pub fn apply_hit(&mut self, mut hit: HitResult, current_tick: u128) {
        if hit.causes.shock {
            self.shock = true;
            self.last_shock_out_roll = current_tick;
        }
        self.wounds.append(&mut hit.causes.wounds);
    }

    pub fn is_dead(&self) -> bool {
        let wounds_limit = if self.wild_card { 3 } else { 0 };
        self.wounds.len() > wounds_limit
    }

    pub fn can_try_to_shock_out(&self, current_tick: u128) -> bool {
        self.shock && (current_tick - self.last_shock_out_roll) >= 10
    }

    pub fn try_to_shock_out(&mut self, current_tick: u128) -> bool {
        self.last_shock_out_roll = current_tick;
        let mut roll = self
            .get_attribute_with_modifiers(Attribute::Spirit)
            .roll_explosive();
        if self.wild_card {
            let wild_roll =
                DiceWithModifier::new(Dice::D6, -(self.wounds.len() as i8)).roll_explosive();
            roll = roll.max(wild_roll);
        }
        if roll >= 4 {
            self.shock = false;
            true
        } else {
            false
        }
    }
}
