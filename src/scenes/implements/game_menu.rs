use tetra::input::Key;

use crate::scenes::helpers::back_btn;
use crate::ui::ButtonBuilder;
use crate::{
    app::App,
    ui::{Alert, Horizontal, Position, SomeUISprites, SomeUISpritesMut, UiSprite, Vertical},
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
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -30.0 },
            },
            &app.assets,
        );
        let settings_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[S] Settings", app.assets.fonts.default.clone())
                .with_keys(vec![Key::S.into()])
                .with_position(Position {
                    x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenterByBottom { offset: 20.0 },
                })
                .with_transition(Transition::Replace(Scene::Settings))
                .build(),
        );
        let quit_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[q] Quit", app.assets.fonts.default.clone())
                .with_keys(vec![Key::Q.into()])
                .with_position(Position {
                    x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenterByBottom { offset: 70.0 },
                })
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
