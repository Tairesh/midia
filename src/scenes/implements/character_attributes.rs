use std::path::Path;

use geometry::Point;
use tetra::{Context, Event};

use crate::{
    app::App,
    colors::Colors,
    game::{races::Personality, traits::Name, Attributes, Avatar, Dice, World},
    savefile,
    savefile::Meta,
    scenes::{
        helpers::{
            back_randomize_next, bg, decorative_label, decorative_label_with_color, easy_back,
            icon_minus, icon_plus, title,
        },
        Scene, SceneImpl, SomeTransitions, Transition,
    },
    ui::{
        Alert, Button, Disable, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut,
        UiSprite, Vertical,
    },
};

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
    attributes: Attributes,
    attributes_points: u8,
    window_size: (i32, i32),
    sprites: [Box<dyn UiSprite>; 31],
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
        );

        Self {
            sprites: [
                bg(&app.assets),
                title(
                    format!("Choose attributes of {}", personality.mind.name),
                    &app.assets,
                ),
                Box::new(Alert::passive(
                    200.0,
                    200.0,
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
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -500.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::AgilityMinus as u8,
                ),
                decorative_label(
                    "d4",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -450.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -400.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::AgilityPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    200.0,
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
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -275.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::SmartsMinus as u8,
                ),
                decorative_label(
                    "d4",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -225.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -175.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::SmartsPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    200.0,
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
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: -50.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::SpiritMinus as u8,
                ),
                decorative_label(
                    "d4",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 50.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::SpiritPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    200.0,
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
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 175.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::StrengthMinus as u8,
                ),
                decorative_label(
                    "d4",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 225.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 275.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::StrengthPlus as u8,
                ),
                Box::new(Alert::passive(
                    200.0,
                    200.0,
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
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 400.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::VigorMinus as u8,
                ),
                decorative_label(
                    "d4",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 450.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 500.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::VigorPlus as u8,
                ),
                decorative_label_with_color(
                    "Points remaining: 5",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                    Colors::DARK_BROWN,
                ),
                back_btn,
                randomize_btn,
                next_btn,
            ],
            attributes: Attributes::default(),
            attributes_points: 5,
            window_size: app.window_size,
            meta,
            personality,
        }
    }

    fn agility_minus(&mut self) -> &mut Button {
        self.sprites[4].as_button().unwrap()
    }
    fn agility_label(&mut self) -> &mut Label {
        self.sprites[5].as_label().unwrap()
    }
    fn agility_plus(&mut self) -> &mut Button {
        self.sprites[6].as_button().unwrap()
    }
    fn smarts_minus(&mut self) -> &mut Button {
        self.sprites[9].as_button().unwrap()
    }
    fn smarts_label(&mut self) -> &mut Label {
        self.sprites[10].as_label().unwrap()
    }
    fn smarts_plus(&mut self) -> &mut Button {
        self.sprites[11].as_button().unwrap()
    }
    fn spirit_minus(&mut self) -> &mut Button {
        self.sprites[14].as_button().unwrap()
    }
    fn spirit_label(&mut self) -> &mut Label {
        self.sprites[15].as_label().unwrap()
    }
    fn spirit_plus(&mut self) -> &mut Button {
        self.sprites[16].as_button().unwrap()
    }
    fn strength_minus(&mut self) -> &mut Button {
        self.sprites[19].as_button().unwrap()
    }
    fn strength_label(&mut self) -> &mut Label {
        self.sprites[20].as_label().unwrap()
    }
    fn strength_plus(&mut self) -> &mut Button {
        self.sprites[21].as_button().unwrap()
    }
    fn vigor_minus(&mut self) -> &mut Button {
        self.sprites[24].as_button().unwrap()
    }
    fn vigor_label(&mut self) -> &mut Label {
        self.sprites[25].as_label().unwrap()
    }
    fn vigor_plus(&mut self) -> &mut Button {
        self.sprites[26].as_button().unwrap()
    }
    fn points_label(&mut self) -> &mut Label {
        self.sprites[27].as_label().unwrap()
    }

    fn update_points(&mut self, ctx: &mut Context) {
        let points = self.attributes_points;
        let window_size = self.window_size;
        self.points_label()
            .update(format!("Points remaining: {points}"), ctx, window_size);
        if points == 0 {
            self.agility_plus().set_disabled(true);
            self.smarts_plus().set_disabled(true);
            self.spirit_plus().set_disabled(true);
            self.strength_plus().set_disabled(true);
            self.vigor_plus().set_disabled(true);
        } else {
            if self.attributes.agility == Dice::D12 {
                self.agility_plus().set_disabled(true);
            } else {
                self.agility_plus().set_disabled(false);
            }
            if self.attributes.smarts == Dice::D12 {
                self.smarts_plus().set_disabled(true);
            } else {
                self.smarts_plus().set_disabled(false);
            }
            if self.attributes.spirit == Dice::D12 {
                self.spirit_plus().set_disabled(true);
            } else {
                self.spirit_plus().set_disabled(false);
            }
            if self.attributes.strength == Dice::D12 {
                self.strength_plus().set_disabled(true);
            } else {
                self.strength_plus().set_disabled(false);
            }
            if self.attributes.vigor == Dice::D12 {
                self.vigor_plus().set_disabled(true);
            } else {
                self.vigor_plus().set_disabled(false);
            }
        }
        if self.attributes.agility == Dice::D4 {
            self.agility_minus().set_disabled(true);
        } else {
            self.agility_minus().set_disabled(false);
        }
        if self.attributes.smarts == Dice::D4 {
            self.smarts_minus().set_disabled(true);
        } else {
            self.smarts_minus().set_disabled(false);
        }
        if self.attributes.spirit == Dice::D4 {
            self.spirit_minus().set_disabled(true);
        } else {
            self.spirit_minus().set_disabled(false);
        }
        if self.attributes.strength == Dice::D4 {
            self.strength_minus().set_disabled(true);
        } else {
            self.strength_minus().set_disabled(false);
        }
        if self.attributes.vigor == Dice::D4 {
            self.vigor_minus().set_disabled(true);
        } else {
            self.vigor_minus().set_disabled(false);
        }
    }

    fn randomize(&mut self, ctx: &mut Context) -> SomeTransitions {
        self.attributes = Attributes::random();
        self.attributes_points = 0;
        let window_size = self.window_size;
        let agility = self.attributes.agility.name();
        let smarts = self.attributes.smarts.name();
        let spirit = self.attributes.spirit.name();
        let strength = self.attributes.strength.name();
        let vigor = self.attributes.vigor.name();
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
            self.attributes.clone(),
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

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        let event = ButtonEvent::from(event);
        match event {
            ButtonEvent::AgilityMinus | ButtonEvent::AgilityPlus => {
                if event == ButtonEvent::AgilityMinus && self.attributes.agility > Dice::D4 {
                    self.attributes_points += 1;
                    self.attributes.agility -= 1;
                } else if event == ButtonEvent::AgilityPlus
                    && self.attributes_points > 0
                    && self.attributes.agility < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.attributes.agility += 1;
                }

                let dice_name = self.attributes.agility.name();
                let window_size = self.window_size;
                self.agility_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SmartsMinus | ButtonEvent::SmartsPlus => {
                if event == ButtonEvent::SmartsMinus && self.attributes.smarts > Dice::D4 {
                    self.attributes_points += 1;
                    self.attributes.smarts -= 1;
                } else if event == ButtonEvent::SmartsPlus
                    && self.attributes_points > 0
                    && self.attributes.smarts < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.attributes.smarts += 1;
                }

                let dice_name = self.attributes.smarts.name();
                let window_size = self.window_size;
                self.smarts_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::SpiritMinus | ButtonEvent::SpiritPlus => {
                if event == ButtonEvent::SpiritMinus && self.attributes.spirit > Dice::D4 {
                    self.attributes_points += 1;
                    self.attributes.spirit -= 1;
                } else if event == ButtonEvent::SpiritPlus
                    && self.attributes_points > 0
                    && self.attributes.spirit < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.attributes.spirit += 1;
                }

                let dice_name = self.attributes.spirit.name();
                let window_size = self.window_size;
                self.spirit_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::StrengthMinus | ButtonEvent::StrengthPlus => {
                if event == ButtonEvent::StrengthMinus && self.attributes.strength > Dice::D4 {
                    self.attributes_points += 1;
                    self.attributes.strength -= 1;
                } else if event == ButtonEvent::StrengthPlus
                    && self.attributes_points > 0
                    && self.attributes.strength < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.attributes.strength += 1;
                }

                let dice_name = self.attributes.strength.name();
                let window_size = self.window_size;
                self.strength_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::VigorMinus | ButtonEvent::VigorPlus => {
                if event == ButtonEvent::VigorMinus && self.attributes.vigor > Dice::D4 {
                    self.attributes_points += 1;
                    self.attributes.vigor -= 1;
                } else if event == ButtonEvent::VigorPlus
                    && self.attributes_points > 0
                    && self.attributes.vigor < Dice::D12
                {
                    self.attributes_points -= 1;
                    self.attributes.vigor += 1;
                }

                let dice_name = self.attributes.vigor.name();
                let window_size = self.window_size;
                self.vigor_label().update(dice_name, ctx, window_size);
                self.update_points(ctx);
                None
            }
            ButtonEvent::Randomize => self.randomize(ctx),
            ButtonEvent::Next => Some(self.next()),
        }
    }
}
