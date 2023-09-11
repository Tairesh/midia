use tetra::{input::Key, Context};

use crate::ui::ButtonBuilder;
use crate::{
    app::App,
    colors::Colors,
    savefile::savefiles_exists,
    ui::{
        Button, Disable, Image, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite,
        Vertical,
    },
    VERSION,
};

use super::super::{helpers::bg, Scene, SceneImpl, Transition};

pub struct MainMenu {
    sprites: [Box<dyn UiSprite>; 7],
}

impl MainMenu {
    pub fn new(app: &App) -> Self {
        let bg = bg(&app.assets);
        let logo = Box::new(Image::new(
            app.assets.images.logo.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByBottom { offset: -100.0 }),
        ));
        let version = Box::new(Label::new(
            VERSION,
            app.assets.fonts.default.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByBottom { offset: -80.0 }),
        ));
        let select_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[e] Select world", app.assets.fonts.default.clone())
                .with_keys(vec![Key::E.into()])
                .with_position(Position::horizontal_center(
                    0.0,
                    Vertical::AtWindowCenterByTop { offset: 0.0 },
                ))
                .with_transition(Transition::Push(Scene::LoadWorld))
                .build()
                .with_disabled(true),
        );
        let create_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[c] Create new world", app.assets.fonts.default.clone())
                .with_keys(vec![Key::C.into()])
                .with_position(Position::horizontal_center(
                    0.0,
                    Vertical::AtWindowCenterByTop { offset: 50.0 },
                ))
                .with_transition(Transition::Push(Scene::CreateWorld))
                .build(),
        );
        let settings_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[s] Settings", app.assets.fonts.default.clone())
                .with_keys(vec![Key::S.into()])
                .with_position(Position::horizontal_center(
                    0.0,
                    Vertical::AtWindowCenterByTop { offset: 100.0 },
                ))
                .with_transition(Transition::Push(Scene::Settings))
                .build(),
        );
        let exit_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[x] Exit", app.assets.fonts.default.clone())
                .with_keys(vec![Key::X.into()])
                .with_position(Position::horizontal_center(
                    0.0,
                    Vertical::AtWindowCenterByTop { offset: 150.0 },
                ))
                .with_transition(Transition::Quit)
                .build(),
        );

        Self {
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg,
                logo,
                version,
                select_btn,
                create_btn,
                settings_btn,
                exit_btn,
            ],
        }
    }

    fn select_btn(&mut self) -> &mut Button {
        self.sprites[3].as_button().unwrap()
    }
}

impl SceneImpl for MainMenu {
    fn on_open(&mut self, _ctx: &mut Context) {
        self.select_btn().set_disabled(!savefiles_exists());
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }
}
