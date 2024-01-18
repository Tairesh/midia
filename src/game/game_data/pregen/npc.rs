use crate::game::{
    races::{Appearance, BodyColor, Gender, Mind, Personality, Race, Sex},
    savage::{Attributes, Skills},
    CharSheet, Dice, MainHand, SkillLevel,
};

pub fn dragan() -> Personality {
    Personality::new(
        Appearance {
            race: Race::Gazan,
            age: 25,
            body_color: Some(BodyColor::LightBrown),
            sex: Sex::Male,
        },
        Mind {
            name: "Dragan".to_string(),
            gender: Gender::Male,
            main_hand: MainHand::Right,
        },
        CharSheet::new(
            true,
            Race::Gazan,
            25,
            Attributes {
                agility: Dice::D8,
                smarts: Dice::D8,
                spirit: Dice::D6,
                strength: Dice::D8,
                vigor: Dice::D6,
            },
            Skills {
                athletics: SkillLevel::D8,
                fighting: SkillLevel::D6,
                shooting: SkillLevel::D8,
                stealth: SkillLevel::D6,
                thievery: SkillLevel::None,
                swimming: SkillLevel::None,
                gambling: SkillLevel::None,
                notice: SkillLevel::D6, // TODO: trait for +2
                survival: SkillLevel::D6,
                healing: SkillLevel::None,
                repair: SkillLevel::None,
                reading: SkillLevel::None,
                persuasion: SkillLevel::D4,
                intimidation: SkillLevel::None,
                climbing: SkillLevel::D8,
                // TODO: add demonology
            },
        ),
    )
}

pub fn grem() -> Personality {
    Personality::new(
        Appearance {
            race: Race::Lagnam,
            age: 23,
            body_color: Some(BodyColor::LightBrown),
            sex: Sex::Male,
        },
        Mind {
            name: "Grem".to_string(),
            gender: Gender::Male,
            main_hand: MainHand::Right,
        },
        CharSheet::new(
            true,
            Race::Lagnam,
            23,
            Attributes {
                agility: Dice::D8,
                smarts: Dice::D6,
                spirit: Dice::D4,
                strength: Dice::D8,
                vigor: Dice::D8,
            },
            Skills {
                athletics: SkillLevel::D4,
                fighting: SkillLevel::D12,
                shooting: SkillLevel::D8,
                stealth: SkillLevel::None,
                thievery: SkillLevel::None,
                swimming: SkillLevel::None,
                gambling: SkillLevel::None,
                notice: SkillLevel::D6,
                survival: SkillLevel::D6,
                healing: SkillLevel::D4,
                repair: SkillLevel::None,
                reading: SkillLevel::None,
                persuasion: SkillLevel::None,
                intimidation: SkillLevel::None,
                climbing: SkillLevel::None,
                // TODO: add weapons crafting skill
            },
        ),
    )
}

pub fn yasma() -> Personality {
    Personality::new(
        Appearance {
            race: Race::Gazan,
            age: 23,
            body_color: Some(BodyColor::Ginger),
            sex: Sex::Male,
        },
        Mind {
            name: "Yasma".to_string(),
            gender: Gender::Male,
            main_hand: MainHand::Ambidexter,
        },
        CharSheet::new(
            true,
            Race::Gazan,
            23,
            Attributes {
                agility: Dice::D10,
                smarts: Dice::D6,
                spirit: Dice::D6,
                strength: Dice::D10,
                vigor: Dice::D4,
            },
            Skills {
                athletics: SkillLevel::None,
                fighting: SkillLevel::D10,
                shooting: SkillLevel::None,
                stealth: SkillLevel::D10,
                thievery: SkillLevel::D4,
                swimming: SkillLevel::None,
                gambling: SkillLevel::D4,
                notice: SkillLevel::D4,
                survival: SkillLevel::None,
                healing: SkillLevel::None,
                repair: SkillLevel::None,
                reading: SkillLevel::None,
                persuasion: SkillLevel::D8,
                intimidation: SkillLevel::D6,
                climbing: SkillLevel::D6,
            },
        ),
    )
}

pub fn shasha() -> Personality {
    Personality::new(
        Appearance {
            race: Race::Nyarnik,
            age: 20,
            body_color: None,
            sex: Sex::Female,
        },
        Mind {
            name: "Shasha".to_string(),
            gender: Gender::Female,
            main_hand: MainHand::Left,
        },
        CharSheet::new(
            true,
            Race::Nyarnik,
            20,
            Attributes {
                agility: Dice::D8,
                smarts: Dice::D6,
                spirit: Dice::D6,
                strength: Dice::D10,
                vigor: Dice::D6,
            },
            Skills {
                athletics: SkillLevel::None,
                fighting: SkillLevel::D12,
                shooting: SkillLevel::None,
                stealth: SkillLevel::None,
                thievery: SkillLevel::None,
                swimming: SkillLevel::None,
                gambling: SkillLevel::D6,
                notice: SkillLevel::None,
                survival: SkillLevel::None,
                healing: SkillLevel::None,
                repair: SkillLevel::None,
                reading: SkillLevel::None,
                persuasion: SkillLevel::D8,
                intimidation: SkillLevel::None,
                climbing: SkillLevel::None,
                // TODO: streetwise
                // TODO: cooking
                // TODO: taunt
            },
        ),
    )
}
