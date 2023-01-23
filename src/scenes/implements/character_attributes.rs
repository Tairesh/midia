use std::collections::HashMap;
use std::path::Path;

use geometry::Point;
use tetra::{Context, Event};

use crate::{
    app::App,
    assets::Assets,
    colors::Colors,
    game::{
        races::Personality, traits::Name, Attribute, Avatar, CharSheet, Dice, Skill, SkillLevel,
        World,
    },
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl From<Attribute> for ButtonEvent {
    fn from(attr: Attribute) -> Self {
        match attr {
            Attribute::Agility => Self::AgilityMinus,
            Attribute::Smarts => Self::SmartsMinus,
            Attribute::Spirit => Self::SpiritMinus,
            Attribute::Strength => Self::StrengthMinus,
            Attribute::Vigor => Self::VigorMinus,
        }
    }
}

impl From<Skill> for ButtonEvent {
    fn from(skill: Skill) -> Self {
        match skill {
            Skill::Athletics => Self::AthleticsMinus,
            Skill::Fighting => Self::FightingMinus,
            Skill::Shooting => Self::ShootingMinus,
            Skill::Stealth => Self::StealthMinus,
            Skill::Thievery => Self::ThieveryMinus,
            Skill::Swimming => Self::SwimmingMinus,
            Skill::Gambling => Self::GamblingMinus,
            Skill::Survival => Self::SurvivalMinus,
            Skill::Healing => Self::HealingMinus,
            Skill::Notice => Self::NoticeMinus,
            Skill::Repair => Self::RepairMinus,
            Skill::Reading => Self::ReadingMinus,
            Skill::Intimidation => Self::IntimidationMinus,
            Skill::Persuasion => Self::PersuasionMinus,
            Skill::Climbing => Self::ClimbingMinus,
        }
    }
}

impl From<u8> for ButtonEvent {
    fn from(n: u8) -> Self {
        unsafe { std::mem::transmute(n) }
    }
}

pub struct CharacterAttributes {
    meta: Meta,
    personality: Personality,
    attributes_points: u8,
    skills_points: i8,
    window_size: (i32, i32),
    sprites: Vec<Box<dyn UiSprite>>,
}

fn attribute_sprites(
    assets: &Assets,
    attr: Attribute,
    dice: Dice,
    i: usize,
) -> [Box<dyn UiSprite>; 5] {
    let y = 130.0;
    let x = -450.0 + i as f32 * 225.0;
    let minus = ButtonEvent::from(attr) as u8;
    let plus = minus + 1;
    [
        Box::new(Alert::passive(
            200.0,
            140.0,
            assets.alert.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: x },
                y: Vertical::ByTop { y },
            },
        )),
        decorative_label(
            attr.name(),
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: x },
                y: Vertical::ByTop { y: y + 30.0 },
            },
            attr.color(),
        ),
        icon_minus(
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: x - 55.0 },
                y: Vertical::ByCenter { y: y + 100.0 },
            },
            minus,
        ),
        decorative_label(
            dice.name(),
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: x },
                y: Vertical::ByCenter { y: y + 100.0 },
            },
            attr.color(),
        ),
        icon_plus(
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: x + 55.0 },
                y: Vertical::ByCenter { y: y + 100.0 },
            },
            plus,
        ),
    ]
}

fn skill_sprites(
    assets: &Assets,
    attr: Attribute,
    skill: Skill,
    level: SkillLevel,
    i: usize,
) -> [Box<dyn UiSprite>; 4] {
    let j = i % 5;
    let i = i / 5;
    let x = -520.0 + i as f32 * 345.0;
    let y = 415.0 + j as f32 * 50.0;
    let offset_for_third_col = if i == 2 { 50.0 } else { 0.0 };

    let minus = ButtonEvent::from(skill) as u8;
    let plus = minus + 1;
    [
        colored_label(
            skill.name(),
            assets,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: x },
                y: Vertical::ByCenter { y },
            },
            attr.color(),
        ),
        icon_minus(
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: x + 160.0 + offset_for_third_col,
                },
                y: Vertical::ByCenter { y },
            },
            minus,
        ),
        colored_label(
            level.name(),
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: x + 220.0 + offset_for_third_col,
                },
                y: Vertical::ByCenter { y },
            },
            attr.color(),
        ),
        icon_plus(
            assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter {
                    offset: x + 280.0 + offset_for_third_col,
                },
                y: Vertical::ByCenter { y },
            },
            plus,
        ),
    ]
}

impl CharacterAttributes {
    pub fn new(path: &Path, personality: Personality, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();

        let mut sprites: Vec<Box<dyn UiSprite>> = vec![
            bg(&app.assets),
            title(
                format!("Choose attributes & skills of {}", personality.mind.name),
                &app.assets,
            ),
        ];
        for (i, (attr, dice)) in personality
            .char_sheet
            .attributes
            .get_attributes()
            .into_iter()
            .enumerate()
        {
            sprites.extend(attribute_sprites(&app.assets, attr, dice, i));
        }

        sprites.push(decorative_label(
            "Attributes' points left: 5",
            &app.assets,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 95.0 }),
            Colors::DARK_BROWN,
        ));
        sprites.push(decorative_label(
            "Skills' points left: 15",
            &app.assets,
            Position::horizontal_center(0.0, Vertical::ByCenter { y: 320.0 }),
            Colors::DARK_BROWN,
        ));
        sprites.push(Box::new(Alert::passive(
            1100.0,
            285.0,
            app.assets.alert.clone(),
            Position::horizontal_center(0.0, Vertical::ByTop { y: 370.0 }),
        )));

        for (i, (attr, skill, level)) in personality
            .char_sheet
            .skills
            .get_skills_by_attributes()
            .into_iter()
            .enumerate()
        {
            sprites.extend(skill_sprites(&app.assets, attr, skill, level, i));
        }

        sprites.push(decorative_label(
            "Parry: 0",
            &app.assets,
            Position::horizontal_center(-200.0, Vertical::ByCenter { y: 350.0 }),
            Colors::DARK_GREEN,
        ));
        sprites.push(decorative_label(
            "Toughness: 0",
            &app.assets,
            Position::horizontal_center(200.0, Vertical::ByCenter { y: 350.0 }),
            Colors::DARK_VIOLET,
        ));

        sprites.extend(back_randomize_next(
            &app.assets,
            ctx,
            ButtonEvent::Randomize as u8,
            ButtonEvent::Next as u8,
            "Create character",
        ));

        Self {
            sprites,
            attributes_points: 5,
            skills_points: 15,
            window_size: app.window_size,
            meta,
            personality,
        }
    }

    fn attributes_points_label(&mut self) -> &mut Label {
        self.sprites[27].as_label().unwrap()
    }
    fn skills_points_label(&mut self) -> &mut Label {
        self.sprites[28].as_label().unwrap()
    }
    fn parry_label(&mut self) -> &mut Label {
        self.sprites[90].as_label().unwrap()
    }
    fn toughness_label(&mut self) -> &mut Label {
        self.sprites[91].as_label().unwrap()
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
        let skills_points = self.personality.char_sheet.calc_skill_points();
        self.skills_points = skills_points;
        let parry = self.personality.char_sheet.parry();
        let toughness = self.personality.char_sheet.toughness();

        let window_size = self.window_size;
        self.parry_label()
            .update(format!("Parry: {parry}"), ctx, window_size);
        self.toughness_label()
            .update(format!("Toughness: {toughness}"), ctx, window_size);
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

        // TODO: refactor this
        if attributes_points == 0 {
            self.set_buttons_disabled(&[6, 11, 16, 21, 26]);
        } else {
            self.set_buttons_disabled_by_value(
                HashMap::from_iter(vec![
                    (6, self.personality.char_sheet.attributes.agility),
                    (11, self.personality.char_sheet.attributes.smarts),
                    (16, self.personality.char_sheet.attributes.spirit),
                    (21, self.personality.char_sheet.attributes.strength),
                    (26, self.personality.char_sheet.attributes.vigor),
                ]),
                SkillLevel::D12,
            );
        }
        self.set_buttons_disabled_by_value(
            HashMap::from_iter([
                (4, self.personality.char_sheet.attributes.agility),
                (9, self.personality.char_sheet.attributes.smarts),
                (14, self.personality.char_sheet.attributes.spirit),
                (19, self.personality.char_sheet.attributes.strength),
                (24, self.personality.char_sheet.attributes.vigor),
            ]),
            SkillLevel::D4,
        );

        if skills_points == 0 {
            self.set_buttons_disabled(&[
                33, 37, 41, 45, 49, 53, 57, 61, 65, 69, 73, 77, 81, 85, 89,
            ]);
        } else {
            self.set_buttons_disabled_by_value(
                HashMap::from_iter([
                    (33, self.personality.char_sheet.skills.athletics),
                    (37, self.personality.char_sheet.skills.fighting),
                    (41, self.personality.char_sheet.skills.shooting),
                    (45, self.personality.char_sheet.skills.stealth),
                    (49, self.personality.char_sheet.skills.thievery),
                    (53, self.personality.char_sheet.skills.swimming),
                    (57, self.personality.char_sheet.skills.gambling),
                    (61, self.personality.char_sheet.skills.notice),
                    (65, self.personality.char_sheet.skills.survival),
                    (69, self.personality.char_sheet.skills.healing),
                    (73, self.personality.char_sheet.skills.repair),
                    (77, self.personality.char_sheet.skills.reading),
                    (81, self.personality.char_sheet.skills.persuasion),
                    (85, self.personality.char_sheet.skills.intimidation),
                    (89, self.personality.char_sheet.skills.climbing),
                ]),
                SkillLevel::D12,
            );
        }
        self.set_buttons_disabled_by_value(
            HashMap::from_iter([
                (31, self.personality.char_sheet.skills.athletics),
                (35, self.personality.char_sheet.skills.fighting),
                (39, self.personality.char_sheet.skills.shooting),
                (43, self.personality.char_sheet.skills.stealth),
                (47, self.personality.char_sheet.skills.thievery),
                (51, self.personality.char_sheet.skills.swimming),
                (55, self.personality.char_sheet.skills.gambling),
                (59, self.personality.char_sheet.skills.notice),
                (63, self.personality.char_sheet.skills.survival),
                (67, self.personality.char_sheet.skills.healing),
                (71, self.personality.char_sheet.skills.repair),
                (75, self.personality.char_sheet.skills.reading),
                (79, self.personality.char_sheet.skills.persuasion),
                (83, self.personality.char_sheet.skills.intimidation),
                (87, self.personality.char_sheet.skills.climbing),
            ]),
            SkillLevel::None,
        );
    }

    fn randomize(&mut self, ctx: &mut Context) -> SomeTransitions {
        self.personality.char_sheet = CharSheet::random(
            &mut rand::thread_rng(),
            true,
            self.personality.appearance.race,
            self.personality.appearance.age,
        );
        self.attributes_points = 0;
        let window_size = self.window_size;

        for (attribute, value) in self.personality.char_sheet.attributes.get_attributes() {
            self.attribute_label(attribute)
                .update(value.name(), ctx, window_size);
        }
        for (_, skill, value) in self
            .personality
            .char_sheet
            .skills
            .get_skills_by_attributes()
        {
            self.skill_label(skill)
                .update(value.name(), ctx, window_size);
        }

        self.update_points(ctx);
        None
    }

    fn next(&self) -> Vec<Transition> {
        // TODO: traits, skills, etc.
        // TODO: find available starting pos in the world
        let avatar = Avatar::dressed_default(0, self.personality.clone(), Point::new(0, 0));
        let mut world = World::create(self.meta.clone(), avatar).init();
        world.save();

        vec![
            Transition::LoadWorld(self.meta.path.clone()),
            Transition::Replace(Scene::GameScene),
        ]
    }

    fn attribute_labels() -> HashMap<Attribute, usize> {
        HashMap::from([
            (Attribute::Agility, 5),
            (Attribute::Smarts, 10),
            (Attribute::Spirit, 15),
            (Attribute::Strength, 20),
            (Attribute::Vigor, 25),
        ])
    }

    fn attribute_label(&mut self, attribute: Attribute) -> &mut Label {
        let index = *Self::attribute_labels().get(&attribute).unwrap();
        self.sprites[index].as_label().unwrap()
    }

    fn update_attribute_label(&mut self, attribute: Attribute, ctx: &mut Context) {
        let dice_name = self.get_attribute(attribute).name();
        let window_size = self.window_size;
        self.attribute_label(attribute)
            .update(dice_name, ctx, window_size);
        self.update_points(ctx);
    }

    fn skill_labels() -> HashMap<Skill, usize> {
        HashMap::from([
            (Skill::Athletics, 32),
            (Skill::Fighting, 36),
            (Skill::Shooting, 40),
            (Skill::Stealth, 44),
            (Skill::Thievery, 48),
            (Skill::Swimming, 52),
            (Skill::Gambling, 56),
            (Skill::Notice, 60),
            (Skill::Survival, 64),
            (Skill::Healing, 68),
            (Skill::Repair, 72),
            (Skill::Reading, 76),
            (Skill::Persuasion, 80),
            (Skill::Intimidation, 84),
            (Skill::Climbing, 88),
        ])
    }

    fn skill_label(&mut self, skill: Skill) -> &mut Label {
        let index = *Self::skill_labels().get(&skill).unwrap();
        self.sprites[index].as_label().unwrap()
    }

    fn update_skill_label(&mut self, skill: Skill, ctx: &mut Context) {
        let skill_name = self.get_skill(skill).name();
        let window_size = self.window_size;
        self.skill_label(skill).update(skill_name, ctx, window_size);
        self.update_points(ctx);
    }

    fn get_attribute(&self, attribute: Attribute) -> Dice {
        self.personality
            .char_sheet
            .attributes
            .get_attribute(attribute)
    }

    fn set_attribute(&mut self, attribute: Attribute, dice: Dice) {
        self.personality
            .char_sheet
            .attributes
            .set_attribute(attribute, dice);
    }

    fn get_skill(&self, skill: Skill) -> SkillLevel {
        self.personality.char_sheet.skills.get_skill(skill)
    }

    fn set_skill(&mut self, skill: Skill, skill_level: SkillLevel) {
        self.personality
            .char_sheet
            .skills
            .set_skill(skill, skill_level);
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
    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        let event = ButtonEvent::from(event);
        let minus_attribute_events: HashMap<ButtonEvent, Attribute> = HashMap::from([
            (ButtonEvent::AgilityMinus, Attribute::Agility),
            (ButtonEvent::SmartsMinus, Attribute::Smarts),
            (ButtonEvent::SpiritMinus, Attribute::Spirit),
            (ButtonEvent::StrengthMinus, Attribute::Strength),
            (ButtonEvent::VigorMinus, Attribute::Vigor),
        ]);
        let plus_attribute_events: HashMap<ButtonEvent, Attribute> = HashMap::from([
            (ButtonEvent::AgilityPlus, Attribute::Agility),
            (ButtonEvent::SmartsPlus, Attribute::Smarts),
            (ButtonEvent::SpiritPlus, Attribute::Spirit),
            (ButtonEvent::StrengthPlus, Attribute::Strength),
            (ButtonEvent::VigorPlus, Attribute::Vigor),
        ]);
        let minus_skill_events: HashMap<ButtonEvent, Skill> = HashMap::from([
            (ButtonEvent::AthleticsMinus, Skill::Athletics),
            (ButtonEvent::FightingMinus, Skill::Fighting),
            (ButtonEvent::ShootingMinus, Skill::Shooting),
            (ButtonEvent::StealthMinus, Skill::Stealth),
            (ButtonEvent::ThieveryMinus, Skill::Thievery),
            (ButtonEvent::SwimmingMinus, Skill::Swimming),
            (ButtonEvent::GamblingMinus, Skill::Gambling),
            (ButtonEvent::NoticeMinus, Skill::Notice),
            (ButtonEvent::SurvivalMinus, Skill::Survival),
            (ButtonEvent::HealingMinus, Skill::Healing),
            (ButtonEvent::RepairMinus, Skill::Repair),
            (ButtonEvent::ReadingMinus, Skill::Reading),
            (ButtonEvent::PersuasionMinus, Skill::Persuasion),
            (ButtonEvent::IntimidationMinus, Skill::Intimidation),
            (ButtonEvent::ClimbingMinus, Skill::Climbing),
        ]);
        let plus_skill_events: HashMap<ButtonEvent, Skill> = HashMap::from([
            (ButtonEvent::AthleticsPlus, Skill::Athletics),
            (ButtonEvent::FightingPlus, Skill::Fighting),
            (ButtonEvent::ShootingPlus, Skill::Shooting),
            (ButtonEvent::StealthPlus, Skill::Stealth),
            (ButtonEvent::ThieveryPlus, Skill::Thievery),
            (ButtonEvent::SwimmingPlus, Skill::Swimming),
            (ButtonEvent::GamblingPlus, Skill::Gambling),
            (ButtonEvent::NoticePlus, Skill::Notice),
            (ButtonEvent::SurvivalPlus, Skill::Survival),
            (ButtonEvent::HealingPlus, Skill::Healing),
            (ButtonEvent::RepairPlus, Skill::Repair),
            (ButtonEvent::ReadingPlus, Skill::Reading),
            (ButtonEvent::PersuasionPlus, Skill::Persuasion),
            (ButtonEvent::IntimidationPlus, Skill::Intimidation),
            (ButtonEvent::ClimbingPlus, Skill::Climbing),
        ]);

        if let Some(&attribute) = minus_attribute_events.get(&event) {
            let value = self.get_attribute(attribute);
            if value > Dice::D4 {
                self.attributes_points += 1;
                self.set_attribute(attribute, value - 1);
                self.update_attribute_label(attribute, ctx);
            }
        } else if let Some(&attribute) = plus_attribute_events.get(&event) {
            let value = self.get_attribute(attribute);
            if self.attributes_points > 0 && value < Dice::D12 {
                self.attributes_points -= 1;
                self.set_attribute(attribute, value + 1);
                self.update_attribute_label(attribute, ctx);
            }
        } else if let Some(&skill) = minus_skill_events.get(&event) {
            let value = self.get_skill(skill);
            let attribute = self.get_attribute(skill.attribute());
            let cost = if value > attribute.into() { 2 } else { 1 };
            if value > SkillLevel::None {
                self.skills_points += cost;
                self.set_skill(skill, value - 1);
                self.update_skill_label(skill, ctx);
            }
        } else if let Some(&skill) = plus_skill_events.get(&event) {
            let value = self.get_skill(skill);
            let attribute = self.get_attribute(skill.attribute());
            let cost = if value >= attribute.into() { 2 } else { 1 };
            if self.skills_points >= cost && value < SkillLevel::D12 {
                self.skills_points -= cost;
                self.set_skill(skill, value + 1);
                self.update_skill_label(skill, ctx);
            }
        }

        match event {
            ButtonEvent::Randomize => self.randomize(ctx),
            ButtonEvent::Next => Some(self.next()),
            _ => None,
        }
    }
}
