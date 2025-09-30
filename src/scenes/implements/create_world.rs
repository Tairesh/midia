use rand::Rng;
use tetra::{Context, Event};

use crate::{
    app::App,
    savefile,
    ui::{
        Draw, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut, Stringify, TextInput,
        UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{back_randomize_next, bg, easy_back, error_label, label, text_input, title},
    Scene, SceneImpl, Transition,
};

const RANDOMIZE_EVENT: u8 = 1;
const CREATE_EVENT: u8 = 2;

fn random_seed<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.next_u32().to_string()
}

pub struct CreateWorld {
    sprites: [Box<dyn UiSprite>; 12],
}

impl CreateWorld {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let mut rng = rand::rng();

        let [back_btn, randomize_btn, create_btn] = back_randomize_next(
            &app.assets,
            ctx,
            RANDOMIZE_EVENT,
            CREATE_EVENT,
            "Create world",
        );

        Self {
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg(&app.assets),
                title("Create new world:", &app.assets),
                label(
                    "World name:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                        y: Vertical::ByCenter { y: 200.0 },
                    },
                ),
                text_input(
                    "Test world",
                    250.0,
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                        y: Vertical::ByCenter { y: 200.0 },
                    },
                ),
                label(
                    "World seed:",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                ),
                text_input(
                    random_seed(&mut rng).as_str(),
                    250.0,
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                        y: Vertical::ByCenter { y: 270.0 },
                    },
                ),
                back_btn,
                randomize_btn,
                create_btn,
                error_label(
                    "Savefile with this name already exists",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                        y: Vertical::ByBottom { y: 180.0 },
                    },
                ),
                error_label(
                    "Seed shall not be empty!",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                        y: Vertical::ByBottom { y: 250.0 },
                    },
                ),
                error_label(
                    "World name shall not be empty!",
                    &app.assets,
                    Position {
                        x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                        y: Vertical::ByBottom { y: 180.0 },
                    },
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

impl SceneImpl for CreateWorld {
    fn on_update(&mut self, _ctx: &mut Context) -> Option<Transition> {
        if !self.name_input().is_danger() && self.name_empty().visible() {
            self.name_empty().set_visible(false);
        }
        if !self.name_input().is_danger() && self.name_error().visible() {
            self.name_error().set_visible(false);
        }
        if !self.seed_input().is_danger() && self.seed_error().visible() {
            self.seed_error().set_visible(false);
        }
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> Option<Transition> {
        easy_back(&event, self.get_update_context_state())
    }

    fn sprites(&self) -> SomeUISprites<'_> {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> Option<Transition> {
        match event {
            RANDOMIZE_EVENT => {
                self.name_input().set_value("Test world");
                self.seed_input()
                    .set_value(random_seed(&mut rand::rng()).as_str());
                None
            }
            CREATE_EVENT => {
                let seed = self.seed_input().value();
                let name = self.name_input().value();
                if seed.is_empty() {
                    self.seed_input().set_danger(true);
                    self.seed_error().set_visible(true);
                }
                if name.is_empty() {
                    self.name_input().set_danger(true);
                    self.name_empty().set_visible(true);
                    None
                } else {
                    match savefile::create(name.as_str(), seed.as_str()) {
                        Ok(path) => Some(Transition::Replace(Scene::CreateCharacter(path))),
                        Err(err) => match err {
                            savefile::SaveError::System(err) => {
                                panic!("Can't write savefile: {err}")
                            }
                            savefile::SaveError::Serialize(err) => {
                                panic!("Can't save world: {err}")
                            }
                            savefile::SaveError::FileExists => {
                                self.name_input().set_danger(true);
                                self.name_error().set_visible(true);
                                None
                            }
                        },
                    }
                }
            }
            _ => None,
        }
    }
}
