use tetra::{Context, input::Key};

use crate::{
    app::App,
    colors::Colors,
    NAME,
    savefile::savefiles_exists,
    ui::{Button, Disable, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite, Vertical}, VERSION,
};

use super::super::{helpers::bg, Scene, SceneImpl, Transition};

pub struct MainMenu {
    sprites: [Box<dyn UiSprite>; 7],
}

impl MainMenu {
    pub fn new(app: &App) -> Self {
        let bg = bg(&app.assets);
        let logo = Box::new(Label::new(
            NAME,
            app.assets.fonts.logo.clone(),
            Colors::BLANCHED_ALMOND,
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByBottom { offset: -100.0 }),
        ));
        let version = Box::new(Label::new(
            VERSION,
            app.assets.fonts.default.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByBottom { offset: -80.0 }),
        ));
        let select_btn = Box::new(
            Button::text(
                vec![Key::E.into()],
                "[e] Select world",
                app.assets.fonts.default.clone(),
                app.assets.button.clone(),
                Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 0.0 }),
                Transition::Push(Scene::LoadWorld),
            )
                .with_disabled(true),
        );
        let create_btn = Box::new(Button::text(
            vec![Key::C.into()],
            "[c] Create new world",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 50.0 }),
            Transition::Push(Scene::CreateWorld),
        ));
        let settings_btn = Box::new(Button::text(
            vec![Key::S.into()],
            "[s] Settings",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 100.0 }),
            Transition::Push(Scene::Settings),
        ));
        let exit_btn = Box::new(Button::text(
            vec![Key::X.into()],
            "[x] Exit",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 150.0 }),
            Transition::Quit,
        ));

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
