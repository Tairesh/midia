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
        let btn = |text, key: Key, y, transition| {
            Box::new(
                ButtonBuilder::new(app.assets.button.clone())
                    .with_text(text, app.assets.fonts.default.clone())
                    .with_keys(vec![key.into()])
                    .with_position(Position::horizontal_center(
                        Vertical::CenterByBottom,
                        Vec2::new(0.0, y),
                    ))
                    .with_transition(transition)
                    .build(),
            )
        };

        Self {
            sprites: [
                Box::new(Alert::new(
                    200.0,
                    190.0,
                    app.assets.alert.clone(),
                    Position::center(),
                )),
                back_btn(
                    Position::horizontal_center(Vertical::CenterByBottom, Vec2::new(0.0, -30.0)),
                    &app.assets,
                ),
                btn(
                    "[S] Settings",
                    Key::S,
                    20.0,
                    Transition::Switch(SceneKind::Settings),
                ),
                btn("[q] Quit", Key::Q, 70.0, Transition::ExitToMainMenu),
            ],
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
