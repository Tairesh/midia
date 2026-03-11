use std::path::Path;

use roguemetry::Vec2;
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
        draw_sprites, Button, Colorize, Draw, Horizontal, JustMesh, Label, Position, Stringify,
        TextInput, TilesetSprite, UISpritesCollection, UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{
        back_randomize_next, bg, easy_back, error_label, icon_left, icon_minus, icon_plus,
        icon_right, label, subtitle, text_input, title, window_size,
    },
    Scene, SceneKind, Transition,
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

// Sprite indices for easier maintenance
mod idx {
    pub const NAME_INPUT: usize = 4;
    pub const NAME_ERROR: usize = 5;
    pub const RACE_NAME: usize = 8;
    pub const GENDER_INPUT: usize = 12;
    pub const AGE_INPUT: usize = 16;
    pub const COLOR_LABEL: usize = 18;
    pub const COLOR_LEFT: usize = 19;
    pub const COLOR_BG: usize = 20;
    pub const COLOR_NAME: usize = 21;
    pub const COLOR_RIGHT: usize = 22;
    pub const RACE_SPRITE: usize = 23;
}

// Common Y positions for form rows
const Y_NAME: f32 = 200.0;
const Y_RACE: f32 = 250.0;
const Y_GENDER: f32 = 300.0;
const Y_AGE: f32 = 350.0;
const Y_COLOR: f32 = 400.0;

// Common X positions
const X_LABEL_RIGHT: f32 = -60.0;
const X_INPUT_LEFT: f32 = -40.0;
const X_INPUT_CENTER: f32 = 5.0;
const X_CENTER: f32 = 120.0;
const X_RIGHT: f32 = 260.0;

fn label_pos(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByRight,
        Vertical::TopByCenter,
        Vec2::new(X_LABEL_RIGHT, y),
    )
}

fn left_btn_pos(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByLeft,
        Vertical::TopByCenter,
        Vec2::new(X_INPUT_LEFT, y),
    )
}

fn center_pos(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByCenter,
        Vertical::TopByCenter,
        Vec2::new(X_CENTER, y),
    )
}

fn input_pos(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByLeft,
        Vertical::TopByCenter,
        Vec2::new(X_INPUT_CENTER, y),
    )
}

fn right_btn_pos(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByRight,
        Vertical::TopByCenter,
        Vec2::new(X_RIGHT, y),
    )
}

pub struct CreateCharacter {
    meta: Meta,
    sprites: [Box<dyn UiSprite>; 27],
    race: PlayableRace,
    body_color: Option<BodyColor>,
}

impl CreateCharacter {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(path: &Path, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();
        let body_color = BodyColor::Ginger;
        let initial_gender = if meta.time.elapsed().unwrap().as_secs().is_multiple_of(2) {
            "Female"
        } else {
            "Male"
        };

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
                // Name row
                label("Name:", &app.assets, label_pos(Y_NAME)),
                text_input(
                    "",
                    300.0,
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByLeft,
                        Vertical::TopByCenter,
                        Vec2::new(X_INPUT_LEFT, Y_NAME),
                    ),
                ),
                error_label(
                    "Character name shall not be empty!",
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByCenter,
                        Vertical::TopByBottom,
                        Vec2::new(110.0, 180.0),
                    ),
                ),
                // Race row
                label("Race:", &app.assets, label_pos(Y_RACE)),
                icon_left(
                    &app.assets,
                    left_btn_pos(Y_RACE),
                    ButtonEvent::RaceLeft as u8,
                ),
                Box::new(Label::new(
                    "Gazan",
                    app.assets.fonts.header.clone(),
                    Colors::DARK_BROWN,
                    center_pos(Y_RACE),
                )),
                icon_right(
                    &app.assets,
                    right_btn_pos(Y_RACE),
                    ButtonEvent::RaceRight as u8,
                ),
                // Gender row
                label(
                    "Gender:",
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByRight,
                        Vertical::TopByCenter,
                        Vec2::new(X_LABEL_RIGHT, 295.0),
                    ),
                ),
                icon_left(
                    &app.assets,
                    left_btn_pos(Y_GENDER),
                    ButtonEvent::GenderLeft as u8,
                ),
                text_input(initial_gender, 210.0, &app.assets, input_pos(Y_GENDER)),
                icon_right(
                    &app.assets,
                    right_btn_pos(Y_GENDER),
                    ButtonEvent::GenderRight as u8,
                ),
                // Age row
                label("Age:", &app.assets, label_pos(Y_AGE)),
                icon_minus(
                    &app.assets,
                    left_btn_pos(Y_AGE),
                    ButtonEvent::AgeMinus as u8,
                ),
                Box::new(TextInput::int(
                    18,
                    (16, 99),
                    210.0,
                    app.assets.fonts.header.clone(),
                    input_pos(Y_AGE),
                )),
                icon_plus(
                    &app.assets,
                    right_btn_pos(Y_AGE),
                    ButtonEvent::AgePlus as u8,
                ),
                // Body color row
                label("Body color:", &app.assets, label_pos(Y_COLOR)),
                icon_left(
                    &app.assets,
                    left_btn_pos(Y_COLOR),
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
                    Position::new(
                        Horizontal::CenterByCenter,
                        Vertical::TopByCenter,
                        Vec2::new(110.0, Y_COLOR),
                    ),
                )),
                Box::new(Label::new(
                    body_color.name(),
                    app.assets.fonts.header.clone(),
                    body_color.text_color(),
                    Position::new(
                        Horizontal::CenterByCenter,
                        Vertical::TopByCenter,
                        Vec2::new(110.0, Y_COLOR),
                    ),
                )),
                icon_right(
                    &app.assets,
                    right_btn_pos(Y_COLOR),
                    ButtonEvent::ColorRight as u8,
                ),
                // Race preview sprite
                Box::new(TilesetSprite::new(
                    Race::Gazan.looks_like(),
                    app.assets.tileset.clone(),
                    Position::new(
                        Horizontal::CenterByCenter,
                        Vertical::TopByCenter,
                        Vec2::new(50.0, Y_RACE),
                    ),
                    3.0,
                    Some(body_color.into()),
                )),
                back_btn,
                randomize_btn,
                next_btn,
            ],
            meta,
            race: PlayableRace::Gazan,
            body_color: Some(body_color),
        }
    }

    // Sprite accessors using index constants
    fn name_input(&mut self) -> &mut TextInput {
        self.sprites[idx::NAME_INPUT].as_text_input().unwrap()
    }
    fn name_empty(&mut self) -> &mut Label {
        self.sprites[idx::NAME_ERROR].as_label().unwrap()
    }
    fn race_name(&mut self) -> &mut Label {
        self.sprites[idx::RACE_NAME].as_label().unwrap()
    }
    fn gender_input(&mut self) -> &mut TextInput {
        self.sprites[idx::GENDER_INPUT].as_text_input().unwrap()
    }
    fn age_input(&mut self) -> &mut TextInput {
        self.sprites[idx::AGE_INPUT].as_text_input().unwrap()
    }
    fn color_label(&mut self) -> &mut Label {
        self.sprites[idx::COLOR_LABEL].as_label().unwrap()
    }
    fn color_left(&mut self) -> &mut Button {
        self.sprites[idx::COLOR_LEFT].as_button().unwrap()
    }
    fn color_bg(&mut self) -> &mut JustMesh {
        self.sprites[idx::COLOR_BG].as_just_mesh().unwrap()
    }
    fn color_name(&mut self) -> &mut Label {
        self.sprites[idx::COLOR_NAME].as_label().unwrap()
    }
    fn color_right(&mut self) -> &mut Button {
        self.sprites[idx::COLOR_RIGHT].as_button().unwrap()
    }
    fn race_sprite(&mut self) -> &mut TilesetSprite {
        self.sprites[idx::RACE_SPRITE].as_tileset_sprite().unwrap()
    }

    fn hide_color_selectors(&mut self, hide: bool) {
        let visible = !hide;
        self.color_label().set_visible(visible);
        self.color_left().set_visible(visible);
        self.color_bg().set_visible(visible);
        self.color_name().set_visible(visible);
        self.color_right().set_visible(visible);
    }

    fn update_color_display(&mut self, color: BodyColor, ctx: &mut Context, win_size: Vec2) {
        self.color_bg().set_color(color);
        self.color_name().update(color.name(), ctx, win_size);
        self.color_name().set_color(color.text_color());
        self.race_sprite().set_color(color);
    }

    fn update_race_display(&mut self, ctx: &mut Context) {
        let win_size = window_size(ctx);
        let race = Race::from(self.race);
        let race_name = self.race.name();

        self.race_name().update(race_name, ctx, win_size);
        self.race_sprite().set_sprite(race.looks_like());
        self.hide_color_selectors(!race.has_custom_colors());

        if race.has_custom_colors() {
            let color = *race.custom_colors().first().unwrap();
            self.body_color = Some(color);
            self.update_color_display(color, ctx, win_size);
        } else {
            self.body_color = None;
            self.race_sprite().remove_color();
        }
    }

    fn randomize(&mut self, ctx: &mut Context) {
        let character = PlayerPersonality::random_playable(&mut rand::rng());

        self.gender_input().set_value(character.mind.gender);
        self.name_input().set_value(character.mind.name);
        self.age_input()
            .set_value(character.appearance.age.to_string());
        self.race = character.appearance.race.into();
        self.body_color = character.appearance.body_color;

        let win_size = window_size(ctx);
        let race = Race::from(self.race);
        let race_name = self.race.name();

        self.race_name().update(race_name, ctx, win_size);
        self.hide_color_selectors(!race.has_custom_colors());
        self.race_sprite().set_sprite(race.looks_like());

        match self.body_color {
            Some(color) => self.update_color_display(color, ctx, win_size),
            None => self.race_sprite().remove_color(),
        }
    }

    fn create(&mut self) -> Transition {
        let name = self.name_input().value();
        if name.is_empty() {
            self.name_input().set_danger(true);
            self.name_empty().set_visible(true);
            return Transition::None;
        }

        let gender: Gender = self.gender_input().value().into();
        let age = self.age_input().value().parse::<u8>().unwrap();
        let race = Race::from(self.race);

        Transition::Push(SceneKind::CharacterAttributes(
            self.meta.path.clone(),
            PlayerPersonality::new(
                Appearance {
                    body_color: self.body_color,
                    sex: Sex::from(&gender),
                    race,
                    age,
                },
                Mind { name, gender },
                CharSheet::default(true, race),
            ),
        ))
    }

    fn handle_race_change(&mut self, ctx: &mut Context, go_next: bool) {
        self.race = if go_next {
            self.race.next()
        } else {
            self.race.prev()
        };
        self.update_race_display(ctx);
    }

    fn handle_color_change(&mut self, ctx: &mut Context, go_next: bool) {
        if let Some(body_color) = self.body_color {
            let colors = Race::from(self.race).custom_colors();
            let new_color = next_color(body_color, &colors, go_next);
            self.body_color = Some(new_color);
            let win_size = window_size(ctx);
            self.update_color_display(new_color, ctx, win_size);
        }
    }

    fn handle_gender_change(&mut self) {
        let input = self.gender_input();
        input.set_value(if input.value() == "Male" {
            "Female"
        } else {
            "Male"
        });
    }

    fn handle_age_change(&mut self, increase: bool) {
        let input = self.age_input();
        if let Ok(value) = input.value().parse::<u8>() {
            let delta = if increase { 1 } else { -1i8 };
            let new_value = (value as i8 + delta).clamp(16, 99) as u8;
            input.set_value(new_value.to_string());
        }
    }
}

impl Scene for CreateCharacter {
    fn on_update(&mut self, _ctx: &mut Context) -> Transition {
        if !self.name_input().is_danger() && self.name_empty().visible() {
            self.name_empty().set_visible(false);
        }
        Transition::None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> Transition {
        if self.sprites.iter().any(|s| s.focused()) {
            return Transition::None;
        }
        easy_back(&event)
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw_sprites(ctx, &mut self.sprites);
    }

    fn sprites_mut(&mut self) -> UISpritesCollection<'_> {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> Transition {
        match ButtonEvent::from(event) {
            ButtonEvent::RaceLeft => self.handle_race_change(ctx, false),
            ButtonEvent::RaceRight => self.handle_race_change(ctx, true),
            ButtonEvent::GenderLeft | ButtonEvent::GenderRight => self.handle_gender_change(),
            ButtonEvent::AgeMinus => self.handle_age_change(false),
            ButtonEvent::AgePlus => self.handle_age_change(true),
            ButtonEvent::ColorLeft => self.handle_color_change(ctx, false),
            ButtonEvent::ColorRight => self.handle_color_change(ctx, true),
            ButtonEvent::Randomize => self.randomize(ctx),
            ButtonEvent::Next => return self.create(),
        }
        Transition::None
    }
}
