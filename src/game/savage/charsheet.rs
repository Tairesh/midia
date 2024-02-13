use rand::Rng;

use crate::game::RollResult;

use super::{
    super::Race, Attribute, Attributes, Dice, DiceWithModifier, HitResult, Skill, SkillLevel,
    Skills, Wound,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CharSheet {
    #[serde(default)]
    pub wild_card: bool,
    pub race: Race,
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
    pub fn new(wild_card: bool, race: Race, attributes: Attributes, skills: Skills) -> Self {
        Self {
            wild_card,
            race,
            attributes,
            skills,
            wounds: vec![],
            shock: false,
            last_shock_out_roll: 0,
        }
    }

    pub fn default(wild_card: bool, race: Race) -> Self {
        let attributes = Attributes::default();
        let skills = Skills::default(race);
        Self::new(wild_card, race, attributes, skills)
    }

    pub fn reset(&mut self) {
        self.wounds.clear();
        self.shock = false;
        self.last_shock_out_roll = 0;
        self.attributes = Attributes::default();
        self.skills = Skills::default(self.race);
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R, wild_card: bool, race: Race) -> Self {
        let attributes = Attributes::random(rng);
        let skills = Skills::random(rng, &attributes, race);
        Self::new(wild_card, race, attributes, skills)
    }

    pub fn calc_skill_points(&self) -> i8 {
        self.skills.calc_skill_points(&self.attributes, self.race)
    }

    pub fn parry(&self) -> u8 {
        let fighting_skill = self.skills.get_skill(Skill::Fighting);
        2 + if fighting_skill > SkillLevel::None {
            fighting_skill.value() / 2
        } else {
            0
        }
    }

    pub fn toughness(&self) -> u8 {
        2 + self.attributes.get_attribute(Attribute::Vigor).value() / 2
    }

    pub fn walk_koeff(&self) -> f32 {
        // TODO: write tests for moving for different races and wounds
        let mut k_wounds = 1.0;
        for &wound in &self.wounds {
            if wound == Wound::LeftLeg || wound == Wound::RightLeg {
                k_wounds *= 0.75;
            }
        }
        let k_shock = if self.shock { 0.5 } else { 1.0 };

        k_wounds * k_shock * self.race.walk_koeff()
    }

    pub fn get_attribute_with_modifiers(&self, attribute: Attribute) -> DiceWithModifier {
        let mut attr_level = self.attributes.get_attribute(attribute);
        match attribute {
            Attribute::Vigor => {
                if self.wounds.contains(&Wound::BatteredGuts) {
                    attr_level -= 1;
                }
            }
            Attribute::Agility => {
                if self.wounds.contains(&Wound::BrokenGuts) {
                    attr_level -= 1;
                }
            }
            Attribute::Strength => {
                if self.wounds.contains(&Wound::BustedGuts) {
                    attr_level -= 1;
                }
            }
            Attribute::Smarts => {
                if self.wounds.contains(&Wound::BrainDamage) {
                    attr_level -= 1;
                }
            }
            Attribute::Spirit => {}
        }
        let mut dice = DiceWithModifier::from(attr_level);
        dice.1 -= self.wounds.len() as i8;
        dice
    }

    pub fn get_skill_with_modifiers(&self, skill: Skill) -> DiceWithModifier {
        DiceWithModifier::from(self.skills.get_skill(skill))
            .with_modifier(-(self.wounds.len() as i8))
    }

    pub fn roll_skill(&self, skill: Skill, modifier: i8) -> RollResult {
        let skill_dice = self.get_skill_with_modifiers(skill).with_modifier(modifier);
        let roll = skill_dice.roll_explosive();

        if self.wild_card && roll.natural != 1 {
            let wild_dice = DiceWithModifier::new(Dice::D6, skill_dice.modifier());
            let wild_roll = wild_dice.roll_explosive();
            if wild_roll.total > roll.total {
                wild_roll
            } else {
                roll
            }
        } else {
            roll
        }
    }

    pub fn apply_hit(&mut self, mut hit: HitResult, current_tick: u128) {
        if hit.consequences.shock {
            self.shock = true;
            self.last_shock_out_roll = current_tick;
        }
        self.wounds.append(&mut hit.consequences.wounds);
    }

    pub fn is_dead(&self) -> bool {
        let wounds_limit = if self.wild_card { 3 } else { 0 };
        self.wounds.len() > wounds_limit
    }

    fn can_try_to_shock_out(&self, current_tick: u128) -> bool {
        self.shock && (current_tick - self.last_shock_out_roll) >= 10
    }

    pub fn try_to_shock_out(&mut self, current_tick: u128) -> bool {
        if !self.can_try_to_shock_out(current_tick) {
            return false;
        }
        self.last_shock_out_roll = current_tick;
        let mut roll = self
            .get_attribute_with_modifiers(Attribute::Spirit)
            .roll_explosive();
        if self.wild_card && roll.natural != 1 {
            let wild_roll =
                DiceWithModifier::new(Dice::D6, -(self.wounds.len() as i8)).roll_explosive();
            if wild_roll.total > roll.total {
                roll = wild_roll;
            }
        }
        if roll.success() {
            self.shock = false;
            true
        } else {
            false
        }
    }
}
