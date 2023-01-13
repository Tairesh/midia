use std::collections::HashMap;
use std::path::Path;

use geometry::Point;
use tetra::graphics::Color;
use tetra::{Context, Event};

use crate::{
    app::App,
    colors::Colors,
    game::{races::Personality, traits::Name, Avatar, CharSheet, Dice, SkillLevel, World},
    savefile::{self, Meta},
    scenes::{
        helpers::{
            back_randomize_next, bg, colored_label, decorative_label, easy_back, icon_minus,
            icon_plus, title,
        },
        Scene, SceneImpl, SomeTransitions, Transition,
    },
    ui::{
        Alert, Disable, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite,
        Vertical,
    },
};

const AGILITY_COLOR: Color = Colors::LIME_GREEN;
const SMARTS_COLOR: Color = Colors::LIGHT_SKY_BLUE;
const SPIRIT_COLOR: Color = Colors::LIGHT_GOLDEN_ROD_YELLOW;
const STRENGTH_COLOR: Color = Colors::ORANGE_RED;
const VIGOR_COLOR: Color = Colors::VIOLET;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ButtonEvent {
    AgilityMinus,
    AgilityPlus,
    SmartsMinus,
    SmartsPlus,
    SpiritMinus,
    SpiritPlus,
    StrengthMinus,
    StrengthPlus,
    VigorMinus,
    VigorPlus,
    AthleticsMinus,
    AthleticsPlus,
    FightingMinus,
    FightingPlus,
    ShootingMinus,
    ShootingPlus,
    StealthMinus,
    StealthPlus,
    ThieveryMinus,
    ThieveryPlus,
    SwimmingMinus,
    SwimmingPlus,
    GamblingMinus,
    GamblingPlus,
    SurvivalMinus,
    SurvivalPlus,
    HealingMinus,
    HealingPlus,
    NoticeMinus,
    NoticePlus,
    RepairMinus,
    RepairPlus,
    ReadingMinus,
    ReadingPlus,
    IntimidationMinus,
    IntimidationPlus,
    PersuasionMinus,
    PersuasionPlus,
    ClimbingMinus,
    ClimbingPlus,
    Randomize,
    Next,
}

impl From<u8> for ButtonEvent {
    fn from(n: u8) -> Self {
        unsafe { std::mem::transmute(n) }
    }
}

pub struct CharacterAttributes {
    meta: Meta,
    personality: Personality,
    char_sheet: CharSheet,
    attributes_points: u8,
    skills_points: u8,
    window_size: (i32, i32),
    sprites: [Box<dyn UiSprite>; 93],
}

impl CharacterAttributes {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(path: &Path, personality: Personality, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();
        let (back_btn, randomize_btn, next_btn) = back_randomize_next(
            &app.assets,
            ctx,
            ButtonEvent::Randomize as u8,
            ButtonEvent::Next as u8,
            "Create character",
        );
        // TODO: races traits and skills
        let char_sheet = CharSheet::default();

        Self {
            sprites: [
                bg(&app.assets),
                title(
                    format!("Choose attributes & skills of {}", personality.mind.name),
                    &app.assets,
                ),
                Box::new(Alert::passive(
                    200.0,
                    150.0,
                    app.assets.alert.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -450.0 },
                        y: Vertical::ByTop { y: 170.0 },
                    },
                )),
                decorative_label(
                    "Agility",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -450.0 },
                        y: Vertical::ByTop { y: 200.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -505.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::AgilityMinus as u8,
                ),
                decorative_label(
                    char_sheet.attributes.agility.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -450.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -395.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::AgilityPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    150.0,
                    app.assets.alert.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -225.0 },
                        y: Vertical::ByTop { y: 170.0 },
                    },
                )),
                decorative_label(
                    "Smarts",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -225.0 },
                        y: Vertical::ByTop { y: 200.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -280.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::SmartsMinus as u8,
                ),
                decorative_label(
                    char_sheet.attributes.smarts.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -225.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -170.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::SmartsPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    150.0,
                    app.assets.alert.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByTop { y: 170.0 },
                    },
                )),
                decorative_label(
                    "Spirit",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByTop { y: 200.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -55.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::SpiritMinus as u8,
                ),
                decorative_label(
                    char_sheet.attributes.spirit.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 55.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::SpiritPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    150.0,
                    app.assets.alert.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 225.0 },
                        y: Vertical::ByTop { y: 170.0 },
                    },
                )),
                decorative_label(
                    "Strength",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 225.0 },
                        y: Vertical::ByTop { y: 200.0 },
                    },
                    STRENGTH_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 170.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::StrengthMinus as u8,
                ),
                decorative_label(
                    char_sheet.attributes.strength.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 225.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    STRENGTH_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 280.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::StrengthPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    150.0,
                    app.assets.alert.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 450.0 },
                        y: Vertical::ByTop { y: 170.0 },
                    },
                )),
                decorative_label(
                    "Vigor",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 450.0 },
                        y: Vertical::ByTop { y: 200.0 },
                    },
                    VIGOR_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 395.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::VigorMinus as u8,
                ),
                decorative_label(
                    char_sheet.attributes.vigor.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 450.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    VIGOR_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 505.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                    ButtonEvent::VigorPlus as u8,
                ),
                decorative_label(
                    "Attributes' points left: 5",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByCenter { y: 150.0 },
                    },
                    Colors::DARK_BROWN,
                ),
                decorative_label(
                    "Skills' points left: 15",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByCenter { y: 350.0 },
                    },
                    Colors::DARK_BROWN,
                ),
                Box::new(Alert::passive(
                    1100.0,
                    285.0,
                    app.assets.alert.clone(),
                    Position::horizontal_center(0.0, Vertical::ByTop { y: 370.0 }),
                )),
                colored_label(
                    "Athletics:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -520.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -360.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::AthleticsMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.athletics.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -300.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -240.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::AthleticsPlus as u8,
                ),
                colored_label(
                    "Fighting:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -520.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -360.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::FightingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.fighting.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -300.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -240.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::FightingPlus as u8,
                ),
                colored_label(
                    "Shooting:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -520.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -360.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::ShootingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.shooting.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -300.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -240.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::ShootingPlus as u8,
                ),
                colored_label(
                    "Stealth:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -520.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -360.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::StealthMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.stealth.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -300.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -240.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::StealthPlus as u8,
                ),
                colored_label(
                    "Thievery:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -520.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -360.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::ThieveryMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.thievery.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -300.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -240.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::ThieveryPlus as u8,
                ),
                colored_label(
                    "Swimming:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -180.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::SwimmingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.swimming.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 40.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    AGILITY_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 100.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::SwimmingPlus as u8,
                ),
                colored_label(
                    "Gambling:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -180.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::GamblingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.gambling.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 40.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 100.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::GamblingPlus as u8,
                ),
                colored_label(
                    "Notice:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -180.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::NoticeMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.notice.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 40.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 100.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::NoticePlus as u8,
                ),
                colored_label(
                    "Survival:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -180.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::SurvivalMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.survival.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 40.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 100.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::SurvivalPlus as u8,
                ),
                colored_label(
                    "Healing:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -180.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -20.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::HealingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.healing.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 40.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 100.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::HealingPlus as u8,
                ),
                colored_label(
                    "Repair:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 170.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 380.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::RepairMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.repair.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 440.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 415.0 },
                    },
                    ButtonEvent::RepairPlus as u8,
                ),
                colored_label(
                    "Reading:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 170.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 380.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::ReadingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.reading.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 440.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    SMARTS_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 465.0 },
                    },
                    ButtonEvent::ReadingPlus as u8,
                ),
                colored_label(
                    "Persuasion:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 170.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 380.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::PersuasionMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.persuasion.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 440.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 515.0 },
                    },
                    ButtonEvent::PersuasionPlus as u8,
                ),
                colored_label(
                    "Intimidation:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 170.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 380.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::IntimidationMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.intimidation.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 440.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    SPIRIT_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 565.0 },
                    },
                    ButtonEvent::IntimidationPlus as u8,
                ),
                colored_label(
                    "Climbing:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 170.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    STRENGTH_COLOR,
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 380.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::ClimbingMinus as u8,
                ),
                colored_label(
                    char_sheet.skills.climbing.name(),
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 440.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    STRENGTH_COLOR,
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 615.0 },
                    },
                    ButtonEvent::ClimbingPlus as u8,
                ),
                back_btn,
                randomize_btn,
                next_btn,
            ],
            attributes_points: 5,
            skills_points: 15,
            window_size: app.window_size,
            meta,
            personality,
            char_sheet,
        }
    }

    fn agility_label(&mut self) -> &mut Label {
        self.sprites[5].as_label().unwrap()
    }
    fn smarts_label(&mut self) -> &mut Label {
        self.sprites[10].as_label().unwrap()
    }
    fn spirit_label(&mut self) -> &mut Label {
        self.sprites[15].as_label().unwrap()
    }
    fn strength_label(&mut self) -> &mut Label {
        self.sprites[20].as_label().unwrap()
    }
    fn vigor_label(&mut self) -> &mut Label {
        self.sprites[25].as_label().unwrap()
    }
    fn attributes_points_label(&mut self) -> &mut Label {
        self.sprites[27].as_label().unwrap()
    }
    fn skills_points_label(&mut self) -> &mut Label {
        self.sprites[28].as_label().unwrap()
    }
    fn athletics_label(&mut self) -> &mut Label {
        self.sprites[32].as_label().unwrap()
    }
    fn fighting_label(&mut self) -> &mut Label {
        self.sprites[36].as_label().unwrap()
    }
    fn shooting_label(&mut self) -> &mut Label {
        self.sprites[40].as_label().unwrap()
    }
    fn stealth_label(&mut self) -> &mut Label {
        self.sprites[44].as_label().unwrap()
    }
    fn thievery_label(&mut self) -> &mut Label {
        self.sprites[48].as_label().unwrap()
    }
    fn swimming_label(&mut self) -> &mut Label {
        self.sprites[52].as_label().unwrap()
    }
    fn gambling_label(&mut self) -> &mut Label {
        self.sprites[56].as_label().unwrap()
    }
    fn notice_label(&mut self) -> &mut Label {
        self.sprites[60].as_label().unwrap()
    }
    fn survival_label(&mut self) -> &mut Label {
        self.sprites[64].as_label().unwrap()
    }
    fn healing_label(&mut self) -> &mut Label {
        self.sprites[68].as_label().unwrap()
    }
    fn repair_label(&mut self) -> &mut Label {
        self.sprites[72].as_label().unwrap()
    }
    fn reading_label(&mut self) -> &mut Label {
        self.sprites[76].as_label().unwrap()
    }
    fn persuasion_label(&mut self) -> &mut Label {
        self.sprites[80].as_label().unwrap()
    }
    fn intimidation_label(&mut self) -> &mut Label {
        self.sprites[84].as_label().unwrap()
    }
    fn climbing_label(&mut self) -> &mut Label {
        self.sprites[88].as_label().unwrap()
    }

    fn set_buttons_disabled(&mut self, sprites: &[usize]) {
        for &i in sprites {
            self.sprites[i].as_button().unwrap().set_disabled(true);
        }
    }

    fn set_buttons_disabled_by_value(
        &mut self,
        data: HashMap<usize, impl Into<SkillLevel>>,
        value: SkillLevel,
    ) {
        for (i, dice) in data {
            let dice: SkillLevel = dice.into();
            self.sprites[i]
                .as_button()
                .unwrap()
                .set_disabled(dice == value);
        }
    }

    fn update_points(&mut self, ctx: &mut Context) {
        let attributes_points = self.attributes_points;
        let skills_points = self.char_sheet.calc_skill_points();
        self.skills_points = skills_points;

        let window_size = self.window_size;
        self.attributes_points_label().update(
            format!("Attributes' points left: {attributes_points}"),
            ctx,
            window_size,
        );
        self.skills_points_label().update(
            format!("Skills' points left: {skills_points}"),
            ctx,
            window_size,
        );

        if attributes_points == 0 {
            self.set_buttons_disabled(&[6, 11, 16, 21, 26]);
        } else {
            self.set_buttons_disabled_by_value(
                HashMap::from_iter(vec![
                    (6, self.char_sheet.attributes.agility),
                    (11, self.char_sheet.attributes.smarts),
                    (16, self.char_sheet.attributes.spirit),
                    (21, self.char_sheet.attributes.strength),
                    (26, self.char_sheet.attributes.vigor),
                ]),
                SkillLevel::D12,
            );
        }
        self.set_buttons_disabled_by_value(
            HashMap::from_iter(vec![
                (4, self.char_sheet.attributes.agility),
                (9, self.char_sheet.attributes.smarts),
                (14, self.char_sheet.attributes.spirit),
                (19, self.char_sheet.attributes.strength),
                (24, self.char_sheet.attributes.vigor),
            ]),
            SkillLevel::D4,
        );

        if skills_points == 0 {
            self.set_buttons_disabled(&[
                33, 37, 41, 45, 49, 53, 57, 61, 65, 69, 73, 77, 81, 85, 89,
            ]);
        } else {
            self.set_buttons_disabled_by_value(
                HashMap::from_iter(vec![
                    (33, self.char_sheet.skills.athletics),
                    (37, self.char_sheet.skills.fighting),
                    (41, self.char_sheet.skills.shooting),
                    (45, self.char_sheet.skills.stealth),
                    (49, self.char_sheet.skills.thievery),
                    (53, self.char_sheet.skills.swimming),
                    (57, self.char_sheet.skills.gambling),
                    (61, self.char_sheet.skills.notice),
                    (65, self.char_sheet.skills.survival),
                    (69, self.char_sheet.skills.healing),
                    (73, self.char_sheet.skills.repair),
                    (77, self.char_sheet.skills.reading),
                    (81, self.char_sheet.skills.persuasion),
                    (85, self.char_sheet.skills.intimidation),
                    (89, self.char_sheet.skills.climbing),
                ]),
                SkillLevel::D12,
            );
        }
        self.set_buttons_disabled_by_value(
            HashMap::from_iter(vec![
                (31, self.char_sheet.skills.athletics),
                (35, self.char_sheet.skills.fighting),
                (39, self.char_sheet.skills.shooting),
                (43, self.char_sheet.skills.stealth),
                (47, self.char_sheet.skills.thievery),
                (51, self.char_sheet.skills.swimming),
                (55, self.char_sheet.skills.gambling),
                (59, self.char_sheet.skills.notice),
                (63, self.char_sheet.skills.survival),
                (67, self.char_sheet.skills.healing),
                (71, self.char_sheet.skills.repair),
                (75, self.char_sheet.skills.reading),
                (79, self.char_sheet.skills.persuasion),
                (83, self.char_sheet.skills.intimidation),
                (87, self.char_sheet.skills.climbing),
            ]),
            SkillLevel::D4_2,
        );
    }

    fn randomize(&mut self, ctx: &mut Context) -> SomeTransitions {
        self.char_sheet = CharSheet::random();
        self.attributes_points = 0;
        let window_size = self.window_size;
        let agility = self.char_sheet.attributes.agility.name();
        let smarts = self.char_sheet.attributes.smarts.name();
        let spirit = self.char_sheet.attributes.spirit.name();
        let strength = self.char_sheet.attributes.strength.name();
        let vigor = self.char_sheet.attributes.vigor.name();
        self.agility_label().update(agility, ctx, window_size);
        self.smarts_label().update(smarts, ctx, window_size);
        self.spirit_label().update(spirit, ctx, window_size);
        self.strength_label().update(strength, ctx, window_size);
        self.vigor_label().update(vigor, ctx, window_size);
        self.update_points(ctx);
        None
    }

    fn next(&self) -> Vec<Transition> {
        // TODO: traits, skills, etc.
        // TODO: find available starting pos in the world
        let avatar = Avatar::dressed_default(
            self.personality.clone(),
            self.char_sheet.clone(),
            Point::new(0, 0),
        );
        let mut world = World::create(self.meta.clone(), avatar).init();
        world.save();

        vec![
            Transition::LoadWorld(self.meta.path.clone()),
            Transition::Replace(Scene::GameScene),
        ]
    }
}

impl SceneImpl for CharacterAttributes {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, false)
    }

    fn on_open(&mut self, ctx: &mut Context) {
        self.update_points(ctx);
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        let event = ButtonEvent::from(event);
        match event {
            ButtonEvent::AgilityMinus | ButtonEvent::AgilityPlus => {
                if event == ButtonEvent::AgilityMinus
                    && self.char_sheet.attributes.agility > Dice::D4
                {
                    self.attributes_points += 1;
                    self.char_sheet.attributes.agility -= 1;
                } else if event == ButtonEvent::AgilityPlus
                    && self.attributes_points > 0
                    && self.char_sheet.attributes.agility < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.char_sheet.attributes.agility += 1;
                }

                let dice_name = self.char_sheet.attributes.agility.name();
                let window_size = self.window_size;
                self.agility_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SmartsMinus | ButtonEvent::SmartsPlus => {
                if event == ButtonEvent::SmartsMinus && self.char_sheet.attributes.smarts > Dice::D4
                {
                    self.attributes_points += 1;
                    self.char_sheet.attributes.smarts -= 1;
                } else if event == ButtonEvent::SmartsPlus
                    && self.attributes_points > 0
                    && self.char_sheet.attributes.smarts < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.char_sheet.attributes.smarts += 1;
                }

                let dice_name = self.char_sheet.attributes.smarts.name();
                let window_size = self.window_size;
                self.smarts_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SpiritMinus | ButtonEvent::SpiritPlus => {
                if event == ButtonEvent::SpiritMinus && self.char_sheet.attributes.spirit > Dice::D4
                {
                    self.attributes_points += 1;
                    self.char_sheet.attributes.spirit -= 1;
                } else if event == ButtonEvent::SpiritPlus
                    && self.attributes_points > 0
                    && self.char_sheet.attributes.spirit < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.char_sheet.attributes.spirit += 1;
                }

                let dice_name = self.char_sheet.attributes.spirit.name();
                let window_size = self.window_size;
                self.spirit_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::StrengthMinus | ButtonEvent::StrengthPlus => {
                if event == ButtonEvent::StrengthMinus
                    && self.char_sheet.attributes.strength > Dice::D4
                {
                    self.attributes_points += 1;
                    self.char_sheet.attributes.strength -= 1;
                } else if event == ButtonEvent::StrengthPlus
                    && self.attributes_points > 0
                    && self.char_sheet.attributes.strength < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.char_sheet.attributes.strength += 1;
                }

                let dice_name = self.char_sheet.attributes.strength.name();
                let window_size = self.window_size;
                self.strength_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::VigorMinus | ButtonEvent::VigorPlus => {
                if event == ButtonEvent::VigorMinus && self.char_sheet.attributes.vigor > Dice::D4 {
                    self.attributes_points += 1;
                    self.char_sheet.attributes.vigor -= 1;
                } else if event == ButtonEvent::VigorPlus
                    && self.attributes_points > 0
                    && self.char_sheet.attributes.vigor < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.char_sheet.attributes.vigor += 1;
                }

                let dice_name = self.char_sheet.attributes.vigor.name();
                let window_size = self.window_size;
                self.vigor_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::AthleticsMinus | ButtonEvent::AthleticsPlus => {
                let cost_plus = if self.char_sheet.skills.athletics
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.athletics
                    > self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::AthleticsMinus
                    && self.char_sheet.skills.athletics > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.athletics -= cost_minus as i8;
                } else if event == ButtonEvent::AthleticsPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.athletics < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.athletics += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.athletics.name();
                let window_size = self.window_size;
                self.athletics_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::FightingMinus | ButtonEvent::FightingPlus => {
                let cost_plus = if self.char_sheet.skills.fighting
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.fighting
                    > self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::FightingMinus
                    && self.char_sheet.skills.fighting > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.fighting -= cost_minus as i8;
                } else if event == ButtonEvent::FightingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.fighting < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.fighting += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.fighting.name();
                let window_size = self.window_size;
                self.fighting_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::ShootingMinus | ButtonEvent::ShootingPlus => {
                let cost_plus = if self.char_sheet.skills.shooting
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.shooting
                    > self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::ShootingMinus
                    && self.char_sheet.skills.shooting > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.shooting -= cost_minus as i8;
                } else if event == ButtonEvent::ShootingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.shooting < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.shooting += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.shooting.name();
                let window_size = self.window_size;
                self.shooting_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::StealthMinus | ButtonEvent::StealthPlus => {
                let cost_plus = if self.char_sheet.skills.stealth
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus =
                    if self.char_sheet.skills.stealth > self.char_sheet.attributes.agility.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::StealthMinus
                    && self.char_sheet.skills.stealth > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.stealth -= cost_minus as i8;
                } else if event == ButtonEvent::StealthPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.stealth < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.stealth += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.stealth.name();
                let window_size = self.window_size;
                self.stealth_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::ThieveryMinus | ButtonEvent::ThieveryPlus => {
                let cost_plus = if self.char_sheet.skills.thievery
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.thievery
                    > self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::ThieveryMinus
                    && self.char_sheet.skills.thievery > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.thievery -= cost_minus as i8;
                } else if event == ButtonEvent::ThieveryPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.thievery < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.thievery += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.thievery.name();
                let window_size = self.window_size;
                self.thievery_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SwimmingMinus | ButtonEvent::SwimmingPlus => {
                let cost_plus = if self.char_sheet.skills.swimming
                    >= self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.swimming
                    > self.char_sheet.attributes.agility.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::SwimmingMinus
                    && self.char_sheet.skills.swimming > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.swimming -= cost_minus as i8;
                } else if event == ButtonEvent::SwimmingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.swimming < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.swimming += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.swimming.name();
                let window_size = self.window_size;
                self.swimming_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::GamblingMinus | ButtonEvent::GamblingPlus => {
                let cost_plus = if self.char_sheet.skills.gambling
                    >= self.char_sheet.attributes.smarts.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus =
                    if self.char_sheet.skills.gambling > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::GamblingMinus
                    && self.char_sheet.skills.gambling > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.gambling -= cost_minus as i8;
                } else if event == ButtonEvent::GamblingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.gambling < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.gambling += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.gambling.name();
                let window_size = self.window_size;
                self.gambling_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::NoticeMinus | ButtonEvent::NoticePlus => {
                let cost_plus =
                    if self.char_sheet.skills.notice >= self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                let cost_minus =
                    if self.char_sheet.skills.notice > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::NoticeMinus
                    && self.char_sheet.skills.notice > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.notice -= cost_minus as i8;
                } else if event == ButtonEvent::NoticePlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.notice < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.notice += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.notice.name();
                let window_size = self.window_size;
                self.notice_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SurvivalMinus | ButtonEvent::SurvivalPlus => {
                let cost_plus = if self.char_sheet.skills.survival
                    >= self.char_sheet.attributes.smarts.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus =
                    if self.char_sheet.skills.survival > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::SurvivalMinus
                    && self.char_sheet.skills.survival > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.survival -= cost_minus as i8;
                } else if event == ButtonEvent::SurvivalPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.survival < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.survival += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.survival.name();
                let window_size = self.window_size;
                self.survival_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::HealingMinus | ButtonEvent::HealingPlus => {
                let cost_plus =
                    if self.char_sheet.skills.healing >= self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                let cost_minus =
                    if self.char_sheet.skills.healing > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::HealingMinus
                    && self.char_sheet.skills.healing > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.healing -= cost_minus as i8;
                } else if event == ButtonEvent::HealingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.healing < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.healing += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.healing.name();
                let window_size = self.window_size;
                self.healing_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::RepairMinus | ButtonEvent::RepairPlus => {
                let cost_plus =
                    if self.char_sheet.skills.repair >= self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                let cost_minus =
                    if self.char_sheet.skills.repair > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::RepairMinus
                    && self.char_sheet.skills.repair > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.repair -= cost_minus as i8;
                } else if event == ButtonEvent::RepairPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.repair < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.repair += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.repair.name();
                let window_size = self.window_size;
                self.repair_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::ReadingMinus | ButtonEvent::ReadingPlus => {
                let cost_plus =
                    if self.char_sheet.skills.reading >= self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                let cost_minus =
                    if self.char_sheet.skills.reading > self.char_sheet.attributes.smarts.into() {
                        2
                    } else {
                        1
                    };
                if event == ButtonEvent::ReadingMinus
                    && self.char_sheet.skills.reading > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.reading -= cost_minus as i8;
                } else if event == ButtonEvent::ReadingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.reading < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.reading += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.reading.name();
                let window_size = self.window_size;
                self.reading_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::PersuasionMinus | ButtonEvent::PersuasionPlus => {
                let cost_plus = if self.char_sheet.skills.persuasion
                    >= self.char_sheet.attributes.spirit.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.persuasion
                    > self.char_sheet.attributes.spirit.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::PersuasionMinus
                    && self.char_sheet.skills.persuasion > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.persuasion -= cost_minus as i8;
                } else if event == ButtonEvent::PersuasionPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.persuasion < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.persuasion += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.persuasion.name();
                let window_size = self.window_size;
                self.persuasion_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::IntimidationMinus | ButtonEvent::IntimidationPlus => {
                let cost_plus = if self.char_sheet.skills.intimidation
                    >= self.char_sheet.attributes.spirit.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.intimidation
                    > self.char_sheet.attributes.spirit.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::IntimidationMinus
                    && self.char_sheet.skills.intimidation > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.intimidation -= cost_minus as i8;
                } else if event == ButtonEvent::IntimidationPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.intimidation < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.intimidation += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.intimidation.name();
                let window_size = self.window_size;
                self.intimidation_label()
                    .update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::ClimbingMinus | ButtonEvent::ClimbingPlus => {
                let cost_plus = if self.char_sheet.skills.climbing
                    >= self.char_sheet.attributes.strength.into()
                {
                    2
                } else {
                    1
                };
                let cost_minus = if self.char_sheet.skills.climbing
                    > self.char_sheet.attributes.strength.into()
                {
                    2
                } else {
                    1
                };
                if event == ButtonEvent::ClimbingMinus
                    && self.char_sheet.skills.climbing > SkillLevel::D4_2
                {
                    self.skills_points += cost_minus;
                    self.char_sheet.skills.climbing -= cost_minus as i8;
                } else if event == ButtonEvent::ClimbingPlus
                    && self.skills_points >= cost_plus
                    && self.char_sheet.skills.climbing < SkillLevel::D12
                {
                    self.skills_points -= cost_plus;
                    self.char_sheet.skills.climbing += cost_plus as i8;
                }

                let dice_name = self.char_sheet.skills.climbing.name();
                let window_size = self.window_size;
                self.climbing_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::Randomize => self.randomize(ctx),
            ButtonEvent::Next => Some(self.next()),
        }
    }
}
