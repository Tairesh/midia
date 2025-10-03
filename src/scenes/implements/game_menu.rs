use roguemetry::Vec2;
use tetra::input::Key;
use tetra::Context;

use crate::scenes::helpers::back_btn;
use crate::{
    app::App,
    ui::{draw_sprites, Alert, ButtonBuilder, Position, UISpritesCollection, UiSprite, Vertical},
};

use super::super::{Scene, SceneKind, Transition};

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
                .with_transition(Transition::Switch(SceneKind::Settings))
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
                .with_transition(Transition::ExitToMainMenu)
                .build(),
        );

        Self {
            sprites: [alert, back_btn, settings_btn, quit_btn],
        }
    }
}

impl Scene for GameMenu {
    fn draw(&mut self, ctx: &mut Context) {
        draw_sprites(ctx, &mut self.sprites);
    }

    fn sprites_mut(&mut self) -> UISpritesCollection<'_> {
        Some(&mut self.sprites)
    }
}
