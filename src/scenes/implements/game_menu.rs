use roguemetry::Vec2;
use tetra::input::Key;

use crate::scenes::helpers::back_btn;
use crate::ui::ButtonBuilder;
use crate::{
    app::App,
    ui::{Alert, Position, SomeUISprites, SomeUISpritesMut, UiSprite, Vertical},
};

use super::super::{Scene, SceneImpl, Transition};

pub struct GameMenu {
    sprites: [Box<dyn UiSprite>; 4],
}

impl GameMenu {
    pub fn new(app: &App) -> Self {
        let alert = Box::new(Alert::new(
            200.0,
            190.0,
            app.assets.alert.clone(),
            Position::center(),
        ));
        let back_btn = back_btn(
            Position::horizontal_center(Vertical::CenterByBottom, Vec2::new(0.0, -30.0)),
            &app.assets,
        );
        let settings_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[S] Settings", app.assets.fonts.default.clone())
                .with_keys(vec![Key::S.into()])
                .with_position(Position::horizontal_center(
                    Vertical::CenterByBottom,
                    Vec2::new(0.0, 20.0),
                ))
                .with_transition(Transition::Replace(Scene::Settings))
                .build(),
        );
        let quit_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[q] Quit", app.assets.fonts.default.clone())
                .with_keys(vec![Key::Q.into()])
                .with_position(Position::horizontal_center(
                    Vertical::CenterByBottom,
                    Vec2::new(0.0, 70.0),
                ))
                .with_transition(Transition::GoMainMenu)
                .build(),
        );

        Self {
            sprites: [alert, back_btn, settings_btn, quit_btn],
        }
    }
}

impl SceneImpl for GameMenu {
    fn sprites(&self) -> SomeUISprites<'_> {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        Some(&mut self.sprites)
    }
}
