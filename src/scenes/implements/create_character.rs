use std::collections::HashMap;
use std::path::Path;

use geometry::{Point, Vec2};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

use crate::ui::{Colorize, JustMesh};
use crate::{
    app::App,
    colors::Colors,
    game::{
        races::{
            Appearance, FurColor, Gender, MainHand, Mind, Personality, PlayableRace, Race, Sex,
        },
        traits::Name,
        Attributes, Avatar, Log, World,
    },
    savefile::{self, GameView, Meta},
    ui::{
        Button, Draw, Horizontal, Label, Position, Positionate, SomeUISprites, SomeUISpritesMut,
        Stringify, TextInput, UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{
        back_btn, bg, easy_back, error_label, icon_left, icon_minus, icon_plus, icon_right, label,
        randomize_btn, subtitle, text_input, title,
    },
    Scene, SceneImpl, SomeTransitions, Transition,
};

#[derive(Debug, Copy, Clone)]
enum ButtonEvent {
    RaceLeft,
    RaceRight,
    GenderLeft,
    GenderRight,
    AgeMinus,
    AgePlus,
    HandLeft,
    HandRight,
    FurLeft,
    FurRight,
    Randomize,
    Create,
}

impl From<u8> for ButtonEvent {
    fn from(n: u8) -> Self {
        unsafe { std::mem::transmute(n) }
    }
}

pub struct CreateCharacter {
    meta: Meta,
    sprites: [Box<dyn UiSprite>; 30],
    race: PlayableRace,
    main_hand: MainHand,
    fur_color: Option<FurColor>,
    window_size: (i32, i32),
}

impl CreateCharacter {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(path: &Path, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();

        let mut randomize_btn = randomize_btn(
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByCenter { y: 500.0 },
            },
            ButtonEvent::Randomize as u8,
        );
        let randomize_btn_size = randomize_btn.calc_size(ctx);
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_btn_size.x / 2.0 - 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            &app.assets,
        );
        let create_btn = Box::new(Button::text(
            vec![(Key::Enter, KeyModifier::Alt).into()],
            "[Alt+Enter] Create",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_btn_size.x / 2.0 + 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(ButtonEvent::Create as u8),
        ));
        let fur_color = FurColor::Gray;

        Self {
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg(&app.assets),
                title("Create new character:", &app.assets),
                subtitle(
                    format!("New adventurer in the «{}» world", meta.name),
                    &app.assets,
                ),
                label(
                    "Name:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 200.0 },
                    },
                ),
                text_input(
                    "",
                    300.0,
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 200.0 },
                    },
                ),
                error_label(
                    "Character name shall not be empty!",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByBottom { y: 180.0 },
                    },
                ),
                label(
                    "Race:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                ),
                Box::new(Button::icon(
                    vec![],
                    "lt",
                    app.assets.tileset.clone(),
                    app.assets.button.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                    Transition::CustomEvent(ButtonEvent::RaceLeft as u8),
                )),
                Box::new(Label::new(
                    "Gazan", // TODO: add icon from tileset
                    app.assets.fonts.header.clone(),
                    Colors::DARK_BROWN,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                )),
                Box::new(Button::icon(
                    vec![],
                    "mt",
                    app.assets.tileset.clone(),
                    app.assets.button.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                    Transition::CustomEvent(ButtonEvent::RaceRight as u8),
                )),
                label(
                    "Gender:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 295.0 },
                    },
                ),
                Box::new(Button::icon(
                    vec![],
                    "lt",
                    app.assets.tileset.clone(),
                    app.assets.button.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    Transition::CustomEvent(ButtonEvent::GenderLeft as u8),
                )),
                text_input(
                    if meta.time.elapsed().unwrap().as_secs() % 2 == 0 {
                        "Female"
                    } else {
                        "Male"
                    },
                    210.0,
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                ),
                Box::new(Button::icon(
                    vec![],
                    "mt",
                    app.assets.tileset.clone(),
                    app.assets.button.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    Transition::CustomEvent(ButtonEvent::GenderRight as u8),
                )),
                label(
                    "Age:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 350.0 },
                    },
                ),
                icon_minus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 350.0 },
                    },
                    ButtonEvent::AgeMinus as u8,
                ),
                Box::new(TextInput::int(
                    18,
                    (16, 99),
                    210.0,
                    app.assets.fonts.header.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                        y: Vertical::ByCenter { y: 350.0 },
                    },
                )),
                icon_plus(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 350.0 },
                    },
                    ButtonEvent::AgePlus as u8,
                ),
                label(
                    "Main hand:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                ),
                icon_left(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                    ButtonEvent::HandLeft as u8,
                ),
                Box::new(Label::new(
                    "Right",
                    app.assets.fonts.header.clone(),
                    Colors::DARK_BROWN,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                )),
                icon_right(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                    ButtonEvent::HandRight as u8,
                ),
                label(
                    "Fur color:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 450.0 },
                    },
                ),
                icon_left(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 450.0 },
                    },
                    ButtonEvent::FurLeft as u8,
                ),
                Box::new(JustMesh::new(
                    Mesh::rounded_rectangle(
                        ctx,
                        ShapeStyle::Fill,
                        Rectangle::new(0.0, 0.0, 200.0, 42.0),
                        BorderRadii::new(5.0),
                    )
                    .unwrap(),
                    Some(fur_color.into()),
                    Vec2::new(200.0, 42.0),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByCenter { y: 450.0 },
                    },
                )),
                Box::new(Label::new(
                    fur_color.name(),
                    app.assets.fonts.header.clone(),
                    fur_color.text_color(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByCenter { y: 450.0 },
                    },
                )),
                icon_right(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 450.0 },
                    },
                    ButtonEvent::FurRight as u8,
                ),
                back_btn,
                randomize_btn,
                create_btn,
            ],
            meta,
            race: PlayableRace::Gazan,
            main_hand: MainHand::Right,
            window_size: app.window_size,
            fur_color: Some(fur_color),
        }
    }

    fn name_input(&mut self) -> &mut TextInput {
        self.sprites[4].as_text_input().unwrap()
    }
    fn name_empty(&mut self) -> &mut Label {
        self.sprites[5].as_label().unwrap()
    }
    fn race_name(&mut self) -> &mut Label {
        self.sprites[8].as_label().unwrap()
    }
    fn gender_input(&mut self) -> &mut TextInput {
        self.sprites[12].as_text_input().unwrap()
    }
    fn age_input(&mut self) -> &mut TextInput {
        self.sprites[16].as_text_input().unwrap()
    }
    fn hand_name(&mut self) -> &mut Label {
        self.sprites[20].as_label().unwrap()
    }
    fn fur_label(&mut self) -> &mut Label {
        self.sprites[22].as_label().unwrap()
    }
    fn fur_left(&mut self) -> &mut Button {
        self.sprites[23].as_button().unwrap()
    }
    fn fur_bg(&mut self) -> &mut JustMesh {
        self.sprites[24].as_just_mesh().unwrap()
    }
    fn fur_name(&mut self) -> &mut Label {
        self.sprites[25].as_label().unwrap()
    }
    fn fur_right(&mut self) -> &mut Button {
        self.sprites[26].as_button().unwrap()
    }

    fn hide_fur_selectors(&mut self, hide: bool) {
        self.fur_label().set_visible(!hide);
        self.fur_left().set_visible(!hide);
        self.fur_bg().set_visible(!hide);
        self.fur_name().set_visible(!hide);
        self.fur_right().set_visible(!hide);
    }

    fn randomize(&mut self, ctx: &mut Context) {
        let mut rng = rand::thread_rng();
        let character = Personality::random(&mut rng, true, true);
        self.gender_input().set_value(character.mind.gender);
        self.name_input().set_value(character.mind.name);
        self.age_input()
            .set_value(character.appearance.age.to_string());
        self.race = character.appearance.race.into();
        self.main_hand = character.mind.main_hand;
        let name = self.main_hand.name();
        let race = self.race.name();
        let window_size = self.window_size;
        self.hand_name().update(name, ctx, window_size);
        self.race_name().update(race, ctx, window_size);
        self.fur_color = character.appearance.fur_color;
        self.hide_fur_selectors(!Race::from(self.race).has_fur());
        if let Some(fur_color) = self.fur_color {
            self.fur_bg().set_color(fur_color);
            self.fur_name().update(fur_color.name(), ctx, window_size);
            self.fur_name().set_color(fur_color.text_color());
        }
    }

    fn create(&mut self) -> SomeTransitions {
        let name = self.name_input().value();
        if name.is_empty() {
            self.name_input().set_danger(true);
            self.name_empty().set_visible(true);
            None
        } else {
            let gender: Gender = self.gender_input().value().into();
            let age = self.age_input().value().parse::<u8>().unwrap();
            let race = Race::from(self.race);
            let character = Personality::new(
                true,
                Appearance {
                    fur_color: self.fur_color,
                    sex: Sex::from(&gender),
                    race,
                    age,
                },
                Mind {
                    name,
                    gender,
                    main_hand: self.main_hand,
                    alive: true,
                },
            );
            // TODO: attributes, traits, skills, etc.
            // TODO: find available starting pos in the world
            let avatar =
                Avatar::dressed_default(character, Attributes::default(), Point::new(0, 0));
            let mut world = World::new(
                self.meta.clone(),
                GameView::default(),
                Log::new(),
                vec![avatar],
                HashMap::new(),
            )
            .init();
            world.save();
            Some(vec![
                Transition::LoadWorld(self.meta.path.clone()),
                Transition::Replace(Scene::GameScene),
            ])
        }
    }
}

impl SceneImpl for CreateCharacter {
    fn on_update(&mut self, _ctx: &mut Context) -> SomeTransitions {
        if !self.name_input().danger() && self.name_empty().visible() {
            self.name_empty().set_visible(false);
        }
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, self.is_there_focused_sprite())
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
            ButtonEvent::RaceLeft | ButtonEvent::RaceRight => {
                self.race = match event {
                    ButtonEvent::RaceLeft => self.race.prev(),
                    ButtonEvent::RaceRight => self.race.next(),
                    _ => unreachable!(),
                };
                let name = self.race.name();
                let window_size = self.window_size;
                self.race_name().update(name, ctx, window_size);
                let has_fur = Race::from(self.race).has_fur();
                self.hide_fur_selectors(!has_fur);
                if has_fur {
                    let fur_color = FurColor::Ginger;
                    self.fur_color = Some(fur_color);
                    let name = fur_color.name();
                    self.fur_name().update(name, ctx, window_size);
                    self.fur_name().set_color(fur_color.text_color());
                    self.fur_bg().set_color(fur_color);
                } else {
                    self.fur_color = None;
                }

                None
            }
            ButtonEvent::GenderLeft | ButtonEvent::GenderRight => {
                let input = self.gender_input();
                let value = input.value();
                input.set_value(if value == "Male" { "Female" } else { "Male" });
                None
            }
            ButtonEvent::AgeMinus | ButtonEvent::AgePlus => {
                let input = self.age_input();
                if let Ok(mut value) = input.value().parse::<u8>() {
                    match event {
                        ButtonEvent::AgeMinus => {
                            value -= 1;
                        }
                        ButtonEvent::AgePlus => {
                            value += 1;
                        }
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{value}"));
                }
                None
            }
            ButtonEvent::HandLeft | ButtonEvent::HandRight => {
                self.main_hand = match event {
                    ButtonEvent::HandRight => self.main_hand.next(),
                    ButtonEvent::HandLeft => self.main_hand.prev(),
                    _ => unreachable!(),
                };
                let name = self.main_hand.name();
                let window_size = self.window_size;
                self.hand_name().update(name, ctx, window_size);
                None
            }
            ButtonEvent::FurLeft | ButtonEvent::FurRight => {
                if let Some(fur_color) = self.fur_color {
                    let fur_color = match event {
                        ButtonEvent::FurLeft => fur_color.prev(),
                        ButtonEvent::FurRight => fur_color.next(),
                        _ => unreachable!(),
                    };
                    self.fur_color = Some(fur_color);
                    let name = fur_color.name();
                    let window_size = self.window_size;
                    self.fur_name().update(name, ctx, window_size);
                    self.fur_name().set_color(fur_color.text_color());
                    self.fur_bg().set_color(fur_color);
                }
                None
            }
            ButtonEvent::Randomize => {
                self.randomize(ctx);
                None
            }
            ButtonEvent::Create => self.create(),
        }
    }
}
