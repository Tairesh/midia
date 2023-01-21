use crate::game::{
    races::{Appearance, FurColor, Gender, Mind, Personality, Race, Sex},
    savage::{Attributes, Skills},
    CharSheet, Dice, MainHand, SkillLevel,
};

pub fn dragan() -> Personality {
    Personality::new(
        Appearance {
            race: Race::Gazan,
            age: 25,
            fur_color: Some(FurColor::LightBrown),
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
                thievery: SkillLevel::D4_2,
                swimming: SkillLevel::D4_2,
                gambling: SkillLevel::D4_2,
                notice: SkillLevel::D6, // TODO: trait for +2
                survival: SkillLevel::D6,
                healing: SkillLevel::D4_2,
                repair: SkillLevel::D4_2,
                reading: SkillLevel::D4_2,
                persuasion: SkillLevel::D4,
                intimidation: SkillLevel::D4_2,
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
            fur_color: Some(FurColor::LightBrown),
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
                stealth: SkillLevel::D4_2,
                thievery: SkillLevel::D4_2,
                swimming: SkillLevel::D4_2,
                gambling: SkillLevel::D4_2,
                notice: SkillLevel::D6,
                survival: SkillLevel::D6,
                healing: SkillLevel::D4,
                repair: SkillLevel::D4_2,
                reading: SkillLevel::D4_2,
                persuasion: SkillLevel::D4_2,
                intimidation: SkillLevel::D4_2,
                climbing: SkillLevel::D4_2,
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
            fur_color: Some(FurColor::Ginger),
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
                athletics: SkillLevel::D4_2,
                fighting: SkillLevel::D10,
                shooting: SkillLevel::D4_2,
                stealth: SkillLevel::D10,
                thievery: SkillLevel::D4,
                swimming: SkillLevel::D4_2,
                gambling: SkillLevel::D4,
                notice: SkillLevel::D4,
                survival: SkillLevel::D4_2,
                healing: SkillLevel::D4_2,
                repair: SkillLevel::D4_2,
                reading: SkillLevel::D4_2,
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
            fur_color: None,
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
                athletics: SkillLevel::D4_2,
                fighting: SkillLevel::D12,
                shooting: SkillLevel::D4_2,
                stealth: SkillLevel::D4_2,
                thievery: SkillLevel::D4_2,
                swimming: SkillLevel::D4_2,
                gambling: SkillLevel::D6,
                notice: SkillLevel::D4_2,
                survival: SkillLevel::D4_2,
                healing: SkillLevel::D4_2,
                repair: SkillLevel::D4_2,
                reading: SkillLevel::D4_2,
                persuasion: SkillLevel::D8,
                intimidation: SkillLevel::D4_2,
                climbing: SkillLevel::D4_2,
                // TODO: streetwise
                // TODO: cooking
                // TODO: taunt
            },
        ),
    )
}
