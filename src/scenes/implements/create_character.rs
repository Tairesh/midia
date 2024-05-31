use std::path::Path;

use geometry::Vec2;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::{Context, Event};

use crate::game::traits::LooksLike;
use crate::{
    app::App,
    colors::Colors,
    game::{
        races::{next_color, BodyColor, Gender, PlayableRace, Race, Sex},
        traits::Name,
        units::{Appearance, Mind, PlayerPersonality},
        CharSheet,
    },
    savefile::{self, Meta},
    ui::{
        Button, Colorize, Draw, Horizontal, JustMesh, Label, Position, SomeUISprites,
        SomeUISpritesMut, Stringify, TextInput, TilesetSprite, UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{
        back_randomize_next, bg, easy_back, error_label, icon_left, icon_minus, icon_plus,
        icon_right, label, subtitle, text_input, title,
    },
    Scene, SceneImpl, Transition,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ButtonEvent {
    RaceLeft,
    RaceRight,
    GenderLeft,
    GenderRight,
    AgeMinus,
    AgePlus,
    ColorLeft,
    ColorRight,
    Randomize,
    Next,
}

impl From<u8> for ButtonEvent {
    fn from(n: u8) -> Self {
        unsafe { std::mem::transmute(n) }
    }
}

pub struct CreateCharacter {
    meta: Meta,
    sprites: [Box<dyn UiSprite>; 27],
    race: PlayableRace,
    body_color: Option<BodyColor>,
    window_size: (i32, i32),
}

impl CreateCharacter {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(path: &Path, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();
        let body_color = BodyColor::Ginger;

        let [back_btn, randomize_btn, next_btn] = back_randomize_next(
            &app.assets,
            ctx,
            ButtonEvent::Randomize as u8,
            ButtonEvent::Next as u8,
            "Next step",
        );

        Self {
            // Order matters, change hardcoded indices in functions below if modified
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
                icon_left(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                    ButtonEvent::RaceLeft as u8,
                ),
                Box::new(Label::new(
                    "Gazan",
                    app.assets.fonts.header.clone(),
                    Colors::DARK_BROWN,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 120.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                )),
                icon_right(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                    ButtonEvent::RaceRight as u8,
                ),
                label(
                    "Gender:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                        y: Vertical::ByCenter { y: 295.0 },
                    },
                ),
                icon_left(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::GenderLeft as u8,
                ),
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
                icon_right(
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                        y: Vertical::ByCenter { y: 300.0 },
                    },
                    ButtonEvent::GenderRight as u8,
                ),
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
                    "Body color:",
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
                    ButtonEvent::ColorLeft as u8,
                ),
                Box::new(JustMesh::new(
                    Mesh::rounded_rectangle(
                        ctx,
                        ShapeStyle::Fill,
                        Rectangle::new(0.0, 0.0, 200.0, 42.0),
                        BorderRadii::new(5.0),
                    )
                    .unwrap(),
                    Some(body_color.into()),
                    Vec2::new(200.0, 42.0),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                        y: Vertical::ByCenter { y: 400.0 },
                    },
                )),
                Box::new(Label::new(
                    body_color.name(),
                    app.assets.fonts.header.clone(),
                    body_color.text_color(),
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
                    ButtonEvent::ColorRight as u8,
                ),
                Box::new(TilesetSprite::new(
                    Race::Gazan.looks_like(),
                    app.assets.tileset.clone(),
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 50.0 },
                        y: Vertical::ByCenter { y: 250.0 },
                    },
                    3.0,
                    Some(body_color.into()),
                )),
                back_btn,
                randomize_btn,
                next_btn,
            ],
            meta,
            race: PlayableRace::Gazan,
            window_size: app.window_size,
            body_color: Some(body_color),
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
    fn color_label(&mut self) -> &mut Label {
        self.sprites[18].as_label().unwrap()
    }
    fn color_left(&mut self) -> &mut Button {
        self.sprites[19].as_button().unwrap()
    }
    fn color_bg(&mut self) -> &mut JustMesh {
        self.sprites[20].as_just_mesh().unwrap()
    }
    fn color_name(&mut self) -> &mut Label {
        self.sprites[21].as_label().unwrap()
    }
    fn color_right(&mut self) -> &mut Button {
        self.sprites[22].as_button().unwrap()
    }
    fn race_sprite(&mut self) -> &mut TilesetSprite {
        self.sprites[23].as_tileset_sprite().unwrap()
    }

    fn hide_color_selectors(&mut self, hide: bool) {
        self.color_label().set_visible(!hide);
        self.color_left().set_visible(!hide);
        self.color_bg().set_visible(!hide);
        self.color_name().set_visible(!hide);
        self.color_right().set_visible(!hide);
    }

    fn randomize(&mut self, ctx: &mut Context) {
        let mut rng = rand::thread_rng();
        let character = PlayerPersonality::random_playable(&mut rng);
        self.gender_input().set_value(character.mind.gender);
        self.name_input().set_value(character.mind.name);
        self.age_input()
            .set_value(character.appearance.age.to_string());
        self.race = character.appearance.race.into();
        let race_name = self.race.name().to_string();
        let window_size = self.window_size;
        self.race_name().update(race_name, ctx, window_size);
        self.body_color = character.appearance.body_color;
        let race = Race::from(self.race);
        self.hide_color_selectors(!race.has_custom_colors());
        self.race_sprite().set_sprite(race.looks_like());
        if let Some(body_color) = self.body_color {
            self.color_bg().set_color(body_color);
            self.color_name()
                .update(body_color.name(), ctx, window_size);
            self.color_name().set_color(body_color.text_color());
            self.race_sprite().set_color(body_color);
        } else {
            self.race_sprite().remove_color();
        }
    }

    fn create(&mut self) -> Option<Transition> {
        let name = self.name_input().value();
        if name.is_empty() {
            self.name_input().set_danger(true);
            self.name_empty().set_visible(true);
            None
        } else {
            let gender: Gender = self.gender_input().value().into();
            let age = self.age_input().value().parse::<u8>().unwrap();
            let race = Race::from(self.race);
            let character = PlayerPersonality::new(
                Appearance {
                    body_color: self.body_color,
                    sex: Sex::from(&gender),
                    race,
                    age,
                },
                Mind { name, gender },
                CharSheet::default(true, race),
            );
            Some(Transition::Push(Scene::CharacterAttributes(
                self.meta.path.clone(),
                character,
            )))
        }
    }
}

impl SceneImpl for CreateCharacter {
    fn on_update(&mut self, _ctx: &mut Context) -> Option<Transition> {
        if !self.name_input().is_danger() && self.name_empty().visible() {
            self.name_empty().set_visible(false);
        }
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> Option<Transition> {
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

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> Option<Transition> {
        let event = ButtonEvent::from(event);
        match event {
            ButtonEvent::RaceLeft | ButtonEvent::RaceRight => {
                self.race = match event {
                    ButtonEvent::RaceLeft => self.race.prev(),
                    ButtonEvent::RaceRight => self.race.next(),
                    _ => unreachable!(),
                };
                let name = self.race.name().to_string();
                let window_size = self.window_size;
                self.race_name().update(name, ctx, window_size);
                let race = Race::from(self.race);
                self.race_sprite().set_sprite(race.looks_like());
                self.hide_color_selectors(!race.has_custom_colors());
                if race.has_custom_colors() {
                    let color = *race.custom_colors().first().unwrap();
                    self.body_color = Some(color);
                    let name = color.name();
                    self.color_name().update(name, ctx, window_size);
                    self.color_name().set_color(color.text_color());
                    self.color_bg().set_color(color);
                    self.race_sprite().set_color(color);
                } else {
                    self.body_color = None;
                    self.race_sprite().remove_color();
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
                // TODO: disable buttons on maximum, minimum values
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
            ButtonEvent::ColorLeft | ButtonEvent::ColorRight => {
                if let Some(body_color) = self.body_color {
                    let colors = Race::from(self.race).custom_colors();
                    let body_color =
                        next_color(body_color, &colors, event == ButtonEvent::ColorRight);
                    self.body_color = Some(body_color);
                    let name = body_color.name();
                    let window_size = self.window_size;
                    self.color_name().update(name, ctx, window_size);
                    self.color_name().set_color(body_color.text_color());
                    self.color_bg().set_color(body_color);
                    self.race_sprite().set_color(body_color);
                }
                None
            }
            ButtonEvent::Randomize => {
                self.randomize(ctx);
                None
            }
            ButtonEvent::Next => self.create(),
        }
    }
}
