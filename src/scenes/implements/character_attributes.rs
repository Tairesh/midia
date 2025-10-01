use std::collections::HashMap;
use std::path::Path;

use roguemetry::{Point, Vec2};
use tetra::{Context, Event};

use crate::game::map::items::helpers::STONE_SPEAR;
use crate::game::Item;
use crate::{
    app::App,
    assets::Assets,
    colors::Colors,
    game::{
        traits::Name,
        units::{Player, PlayerPersonality},
        AttrLevel, Attribute, CharSheet, Skill, SkillLevel, World,
    },
    savefile::{self, Meta},
    scenes::{
        helpers::{
            back_randomize_reset_next, bg, colored_label, decorative_label, easy_back, icon_minus,
            icon_plus, title,
        },
        SceneImpl, Transition,
    },
    ui::{
        Alert, Disable, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite,
        Vertical,
    },
};

// TODO: Move this to separate file

#[allow(dead_code)]
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
    Reset,
}

impl ButtonEvent {
    pub fn is_minus(self) -> bool {
        matches!(
            self,
            Self::AgilityMinus
                | Self::SmartsMinus
                | Self::SpiritMinus
                | Self::StrengthMinus
                | Self::VigorMinus
                | Self::AthleticsMinus
                | Self::FightingMinus
                | Self::ShootingMinus
                | Self::StealthMinus
                | Self::ThieveryMinus
                | Self::SwimmingMinus
                | Self::GamblingMinus
                | Self::SurvivalMinus
                | Self::HealingMinus
                | Self::NoticeMinus
                | Self::RepairMinus
                | Self::ReadingMinus
                | Self::IntimidationMinus
                | Self::PersuasionMinus
                | Self::ClimbingMinus
        )
    }
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

impl TryFrom<ButtonEvent> for Attribute {
    type Error = ();

    fn try_from(value: ButtonEvent) -> Result<Self, Self::Error> {
        match value {
            ButtonEvent::AgilityMinus | ButtonEvent::AgilityPlus => Ok(Self::Agility),
            ButtonEvent::SmartsMinus | ButtonEvent::SmartsPlus => Ok(Self::Smarts),
            ButtonEvent::SpiritMinus | ButtonEvent::SpiritPlus => Ok(Self::Spirit),
            ButtonEvent::StrengthMinus | ButtonEvent::StrengthPlus => Ok(Self::Strength),
            ButtonEvent::VigorMinus | ButtonEvent::VigorPlus => Ok(Self::Vigor),
            _ => Err(()),
        }
    }
}

impl TryFrom<ButtonEvent> for Skill {
    type Error = ();

    fn try_from(value: ButtonEvent) -> Result<Self, Self::Error> {
        match value {
            ButtonEvent::AthleticsMinus | ButtonEvent::AthleticsPlus => Ok(Self::Athletics),
            ButtonEvent::FightingMinus | ButtonEvent::FightingPlus => Ok(Self::Fighting),
            ButtonEvent::ShootingMinus | ButtonEvent::ShootingPlus => Ok(Self::Shooting),
            ButtonEvent::StealthMinus | ButtonEvent::StealthPlus => Ok(Self::Stealth),
            ButtonEvent::ThieveryMinus | ButtonEvent::ThieveryPlus => Ok(Self::Thievery),
            ButtonEvent::SwimmingMinus | ButtonEvent::SwimmingPlus => Ok(Self::Swimming),
            ButtonEvent::GamblingMinus | ButtonEvent::GamblingPlus => Ok(Self::Gambling),
            ButtonEvent::SurvivalMinus | ButtonEvent::SurvivalPlus => Ok(Self::Survival),
            ButtonEvent::HealingMinus | ButtonEvent::HealingPlus => Ok(Self::Healing),
            ButtonEvent::NoticeMinus | ButtonEvent::NoticePlus => Ok(Self::Notice),
            ButtonEvent::RepairMinus | ButtonEvent::RepairPlus => Ok(Self::Repair),
            ButtonEvent::ReadingMinus | ButtonEvent::ReadingPlus => Ok(Self::Reading),
            ButtonEvent::IntimidationMinus | ButtonEvent::IntimidationPlus => {
                Ok(Self::Intimidation)
            }
            ButtonEvent::PersuasionMinus | ButtonEvent::PersuasionPlus => Ok(Self::Persuasion),
            ButtonEvent::ClimbingMinus | ButtonEvent::ClimbingPlus => Ok(Self::Climbing),
            _ => Err(()),
        }
    }
}

pub struct CharacterAttributes {
    meta: Meta,
    personality: PlayerPersonality,
    attributes_points: u8,
    skills_points: i8,
    window_size: (i32, i32),
    sprites: Vec<Box<dyn UiSprite>>,
}

fn attribute_sprites(
    assets: &Assets,
    attr: Attribute,
    dice: AttrLevel,
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
            Position::horizontal_center(Vertical::TopByTop, Vec2::new(x, y)),
        )),
        decorative_label(
            attr.name(),
            assets,
            Position::horizontal_center(Vertical::TopByTop, Vec2::new(x, y + 30.0)),
            attr.color(),
        ),
        icon_minus(
            assets,
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(x - 55.0, y + 100.0)),
            minus,
        ),
        decorative_label(
            dice.name(),
            assets,
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(x, y + 100.0)),
            attr.color(),
        ),
        icon_plus(
            assets,
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(x + 55.0, y + 100.0)),
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
            Position::new(
                Horizontal::CenterByLeft,
                Vertical::TopByCenter,
                Vec2::new(x, y),
            ),
            attr.color(),
        ),
        icon_minus(
            assets,
            Position::horizontal_center(
                Vertical::TopByCenter,
                Vec2::new(x + 160.0 + offset_for_third_col, y),
            ),
            minus,
        ),
        colored_label(
            level.name(),
            assets,
            Position::horizontal_center(
                Vertical::TopByCenter,
                Vec2::new(x + 220.0 + offset_for_third_col, y),
            ),
            attr.color(),
        ),
        icon_plus(
            assets,
            Position::horizontal_center(
                Vertical::TopByCenter,
                Vec2::new(x + 280.0 + offset_for_third_col, y),
            ),
            plus,
        ),
    ]
}

impl CharacterAttributes {
    pub fn new(path: &Path, personality: PlayerPersonality, app: &App, ctx: &mut Context) -> Self {
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
            Position::horizontal_center(Vertical::TopByTop, Vec2::new(0.0, 95.0)),
            Colors::DARK_BROWN,
        ));
        sprites.push(decorative_label(
            "Skills' points left: 15",
            &app.assets,
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(0.0, 320.0)),
            Colors::DARK_BROWN,
        ));
        sprites.push(Box::new(Alert::passive(
            1100.0,
            285.0,
            app.assets.alert.clone(),
            Position::horizontal_center(Vertical::TopByTop, Vec2::new(0.0, 370.0)),
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
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(-200.0, 350.0)),
            Colors::DARK_GREEN,
        ));
        sprites.push(decorative_label(
            "Toughness: 0",
            &app.assets,
            Position::horizontal_center(Vertical::TopByCenter, Vec2::new(200.0, 350.0)),
            Colors::DARK_VIOLET,
        ));

        sprites.extend(back_randomize_reset_next(
            &app.assets,
            ctx,
            ButtonEvent::Randomize as u8,
            ButtonEvent::Reset as u8,
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

    fn update_attributes_and_skills(&mut self, ctx: &mut Context) {
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
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.personality.char_sheet.reset();
        self.attributes_points = 5;
        self.update_attributes_and_skills(ctx);
        self.update_points(ctx);
    }

    fn randomize(&mut self, ctx: &mut Context) {
        self.personality.char_sheet =
            CharSheet::random(&mut rand::rng(), true, self.personality.appearance.race);
        self.attributes_points = 0;
        self.update_attributes_and_skills(ctx);
        self.update_points(ctx);
    }

    fn next(&self) -> Transition {
        // TODO: traits, skills, etc.
        // TODO: find available starting pos in the world
        let mut avatar = Player::new(self.personality.clone(), Point::new(0, 0));
        avatar.inventory.wield.wield(Item::new(STONE_SPEAR));
        let mut world = World::create(self.meta.clone(), avatar);
        world.save();

        Transition::LoadWorld(self.meta.path.clone())
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
        let dice_name = self.get_attribute(attribute).name().to_string();
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
        let skill_name = self.get_skill(skill).name().to_string();
        let window_size = self.window_size;
        self.skill_label(skill).update(skill_name, ctx, window_size);
        self.update_points(ctx);
    }

    fn get_attribute(&self, attribute: Attribute) -> AttrLevel {
        self.personality
            .char_sheet
            .attributes
            .get_attribute(attribute)
    }

    fn set_attribute(&mut self, attribute: Attribute, dice: AttrLevel) {
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
    fn event(&mut self, _ctx: &mut Context, event: Event) -> Option<Transition> {
        easy_back(&event, self.get_update_context_state())
    }

    fn on_open(&mut self, ctx: &mut Context) {
        self.update_points(ctx);
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeUISprites<'_> {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        Some(&mut self.sprites)
    }

    // TODO: refactor and delete this allow
    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> Option<Transition> {
        let event = ButtonEvent::from(event);
        if let Ok(attribute) = Attribute::try_from(event) {
            if event.is_minus() {
                let value = self.get_attribute(attribute);
                if value > AttrLevel::D4 {
                    self.attributes_points += 1;
                    self.set_attribute(attribute, value - 1);
                    self.update_attribute_label(attribute, ctx);
                }
            } else {
                let value = self.get_attribute(attribute);
                if self.attributes_points > 0 && value < AttrLevel::D12 {
                    self.attributes_points -= 1;
                    self.set_attribute(attribute, value + 1);
                    self.update_attribute_label(attribute, ctx);
                }
            }
        } else if let Ok(skill) = Skill::try_from(event) {
            if event.is_minus() {
                let value = self.get_skill(skill);
                let attribute = self.get_attribute(skill.attribute());
                let cost = if value > attribute.into() { 2 } else { 1 };
                if value > SkillLevel::None {
                    self.skills_points += cost;
                    self.set_skill(skill, value - 1);
                    self.update_skill_label(skill, ctx);
                }
            } else {
                let value = self.get_skill(skill);
                let attribute = self.get_attribute(skill.attribute());
                let cost = if value >= attribute.into() { 2 } else { 1 };
                if self.skills_points >= cost && value < SkillLevel::D12 {
                    self.skills_points -= cost;
                    self.set_skill(skill, value + 1);
                    self.update_skill_label(skill, ctx);
                }
            }
        }

        if matches!(event, ButtonEvent::Next) {
            return Some(self.next());
        }

        match event {
            ButtonEvent::Randomize => self.randomize(ctx),
            ButtonEvent::Reset => self.reset(ctx),
            _ => {}
        }

        None
    }
}
