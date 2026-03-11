use rand::Rng;
use roguemetry::Vec2;
use tetra::{Context, Event};

use crate::{
    app::App,
    savefile,
    ui::{
        draw_sprites, Draw, Horizontal, Label, Position, Stringify, TextInput, UISpritesCollection,
        UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{back_randomize_next, bg, easy_back, error_label, label, text_input, title},
    Scene, SceneKind, Transition,
};

const RANDOMIZE_EVENT: u8 = 1;
const CREATE_EVENT: u8 = 2;

fn random_seed<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.next_u32().to_string()
}

fn field_position(y: f32, is_label: bool) -> Position {
    Position::new(
        if is_label {
            Horizontal::CenterByRight
        } else {
            Horizontal::CenterByLeft
        },
        Vertical::TopByCenter,
        Vec2::new(if is_label { -10.0 } else { 0.0 }, y),
    )
}

fn error_position(y: f32) -> Position {
    Position::new(
        Horizontal::CenterByCenter,
        Vertical::TopByBottom,
        Vec2::new(125.0, y),
    )
}

pub struct CreateWorld {
    // TODO: use some struct instead of array
    sprites: [Box<dyn UiSprite>; 12],
}

impl CreateWorld {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let seed = random_seed(&mut rand::rng());
        let [back_btn, randomize_btn, create_btn] = back_randomize_next(
            &app.assets,
            ctx,
            RANDOMIZE_EVENT,
            CREATE_EVENT,
            "Create world",
        );

        Self {
            // Order matters, change hardcoded indices in functions below if modified
            sprites: [
                bg(&app.assets),
                title("Create new world:", &app.assets),
                label("World name:", &app.assets, field_position(200.0, true)),
                text_input(
                    "Test world",
                    250.0,
                    &app.assets,
                    field_position(200.0, false),
                ),
                label("World seed:", &app.assets, field_position(270.0, true)),
                text_input(&seed, 250.0, &app.assets, field_position(270.0, false)),
                back_btn,
                randomize_btn,
                create_btn,
                error_label(
                    "Savefile with this name already exists",
                    &app.assets,
                    error_position(180.0),
                ),
                error_label(
                    "Seed shall not be empty!",
                    &app.assets,
                    error_position(250.0),
                ),
                error_label(
                    "World name shall not be empty!",
                    &app.assets,
                    error_position(180.0),
                ),
            ],
        }
    }

    fn name_input(&mut self) -> &mut TextInput {
        self.sprites[3].as_text_input().unwrap()
    }
    fn name_error(&mut self) -> &mut Label {
        self.sprites[9].as_label().unwrap()
    }
    fn name_empty(&mut self) -> &mut Label {
        self.sprites[11].as_label().unwrap()
    }
    fn seed_input(&mut self) -> &mut TextInput {
        self.sprites[5].as_text_input().unwrap()
    }
    fn seed_error(&mut self) -> &mut Label {
        self.sprites[10].as_label().unwrap()
    }
}

impl Scene for CreateWorld {
    fn on_update(&mut self, _ctx: &mut Context) -> Transition {
        let name_danger = self.name_input().is_danger();
        let seed_danger = self.seed_input().is_danger();

        if !name_danger {
            self.name_empty().set_visible(false);
            self.name_error().set_visible(false);
        }
        if !seed_danger {
            self.seed_error().set_visible(false);
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

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> Transition {
        match event {
            RANDOMIZE_EVENT => {
                self.name_input().set_value("Test world");
                self.seed_input().set_value(random_seed(&mut rand::rng()));
                Transition::None
            }
            CREATE_EVENT => self.try_create_world(),
            _ => Transition::None,
        }
    }
}

impl CreateWorld {
    fn try_create_world(&mut self) -> Transition {
        let seed = self.seed_input().value();
        let name = self.name_input().value();

        if seed.is_empty() {
            self.seed_input().set_danger(true);
            self.seed_error().set_visible(true);
        }
        if name.is_empty() {
            self.name_input().set_danger(true);
            self.name_empty().set_visible(true);
            return Transition::None;
        }

        match savefile::create(&name, &seed) {
            Ok(path) => Transition::Switch(SceneKind::CreateCharacter(path)),
            Err(savefile::SaveError::FileExists) => {
                self.name_input().set_danger(true);
                self.name_error().set_visible(true);
                Transition::None
            }
            Err(savefile::SaveError::System(err)) => panic!("Can't write savefile: {err}"),
            Err(savefile::SaveError::Serialize(err)) => panic!("Can't save world: {err}"),
        }
    }
}
