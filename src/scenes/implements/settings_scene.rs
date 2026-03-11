use roguemetry::Vec2;
use tetra::input::{Key, KeyModifier};
use tetra::window::WindowPosition;
use tetra::{Context, Event};

use crate::{
    app::App,
    settings::Settings,
    ui::{
        draw_sprites, Button, ButtonBuilder, Horizontal, Position, Press, Stringify, TextInput,
        UISpritesCollection, UiSprite, Vertical,
    },
};

use super::super::{
    helpers::{back_btn, bg, easy_back, icon_minus, icon_plus, label, title},
    Scene, Transition,
};

#[derive(Debug, Copy, Clone)]
enum ButtonEvent {
    WindowMode,
    FullscreenMode,
    RepeatIntervalMinus,
    RepeatIntervalPlus,
}

impl From<u8> for ButtonEvent {
    #[inline]
    fn from(n: u8) -> Self {
        match n {
            0 => Self::WindowMode,
            1 => Self::FullscreenMode,
            2 => Self::RepeatIntervalMinus,
            3 => Self::RepeatIntervalPlus,
            _ => unreachable!(),
        }
    }
}

pub struct SettingsScene {
    sprites: [Box<dyn UiSprite>; 10],
}

impl SettingsScene {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let settings = Settings::instance();
        let is_fullscreen = settings.window.fullscreen;

        let fullscreen_btn = Self::mode_button(
            app,
            "[Alt+F] Fullscreen",
            Key::F,
            Horizontal::CenterByLeft,
            100.0,
            ButtonEvent::FullscreenMode,
            is_fullscreen,
        );
        let mut window_btn = Self::mode_button(
            app,
            "[Alt+W] Window",
            Key::W,
            Horizontal::CenterByRight,
            98.0,
            ButtonEvent::WindowMode,
            !is_fullscreen,
        );
        let label_x = 90.0 - window_btn.size(ctx).x;

        Self {
            // Order matters: change hardcoded indices in functions below if modified
            sprites: [
                bg(&app.assets),
                title("Settings", &app.assets),
                fullscreen_btn,
                window_btn,
                label("Window mode:", &app.assets, Self::top_right(label_x, 200.0)),
                label(
                    "Repeat delay:",
                    &app.assets,
                    Self::top_right(label_x, 250.0),
                ),
                icon_minus(
                    &app.assets,
                    Self::top_right(0.0, 250.0),
                    ButtonEvent::RepeatIntervalMinus as u8,
                ),
                Box::new(TextInput::int(
                    settings.input.repeat_interval,
                    (1, 10000),
                    190.0,
                    app.assets.fonts.header.clone(),
                    Self::top_left(5.0, 250.0),
                )),
                icon_plus(
                    &app.assets,
                    Self::top_left(200.0, 250.0),
                    ButtonEvent::RepeatIntervalPlus as u8,
                ),
                back_btn(
                    Position::horizontal_center(Vertical::BottomByBottom, Vec2::new(0.0, -200.0)),
                    &app.assets,
                ),
            ],
        }
    }

    fn mode_button(
        app: &App,
        text: &str,
        key: Key,
        horizontal: Horizontal,
        x: f32,
        event: ButtonEvent,
        pressed: bool,
    ) -> Box<dyn UiSprite> {
        Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text(text, app.assets.fonts.default.clone())
                .with_keys(vec![(key, KeyModifier::Alt).into()])
                .with_position(Position::new(
                    horizontal,
                    Vertical::TopByCenter,
                    Vec2::new(x, 200.0),
                ))
                .with_transition(Transition::CustomEvent(event as u8))
                .with_fixable(true)
                .with_pressed(pressed)
                .build(),
        )
    }

    fn top_right(x: f32, y: f32) -> Position {
        Position::new(
            Horizontal::CenterByRight,
            Vertical::TopByCenter,
            Vec2::new(x, y),
        )
    }

    fn top_left(x: f32, y: f32) -> Position {
        Position::new(
            Horizontal::CenterByLeft,
            Vertical::TopByCenter,
            Vec2::new(x, y),
        )
    }

    fn fullscreen_btn(&mut self) -> &mut Button {
        self.sprites[2].as_button().unwrap()
    }

    fn window_btn(&mut self) -> &mut Button {
        self.sprites[3].as_button().unwrap()
    }

    fn repeat_interval_input(&mut self) -> &mut TextInput {
        self.sprites[7].as_text_input().unwrap()
    }
}

impl Scene for SettingsScene {
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
            ButtonEvent::FullscreenMode => {
                self.window_btn().unpress();
                if !tetra::window::is_fullscreen(ctx) {
                    Settings::instance().window.fullscreen = true;
                    if let Ok((width, height)) = tetra::window::get_current_monitor_size(ctx) {
                        tetra::window::set_size(ctx, width, height).ok();
                    }
                    tetra::window::set_fullscreen(ctx, true).ok();
                }
            }
            ButtonEvent::WindowMode => {
                self.fullscreen_btn().unpress();
                if tetra::window::is_fullscreen(ctx) {
                    Settings::instance().window.fullscreen = false;
                    tetra::window::set_fullscreen(ctx, false).ok();
                    tetra::window::set_decorated(ctx, true);
                    let window_settings = &Settings::instance().window;
                    tetra::window::set_size(ctx, window_settings.width, window_settings.height)
                        .ok();
                    let current_monitor =
                        tetra::window::get_current_monitor(ctx).unwrap_or(0) as i32;
                    tetra::window::set_position(
                        ctx,
                        WindowPosition::Centered(current_monitor),
                        WindowPosition::Centered(current_monitor),
                    );
                }
            }
            ButtonEvent::RepeatIntervalMinus | ButtonEvent::RepeatIntervalPlus => {
                let input = self.repeat_interval_input();
                if let Ok(mut value) = input.value().parse::<u32>() {
                    match ButtonEvent::from(event) {
                        ButtonEvent::RepeatIntervalMinus => value -= 1,
                        ButtonEvent::RepeatIntervalPlus => value += 1,
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{value}"));
                    Settings::instance().input.repeat_interval = value;
                }
            }
        }
        Transition::None
    }
}

impl Drop for SettingsScene {
    fn drop(&mut self) {
        let mut settings = Settings::instance();
        settings.input.repeat_interval =
            self.repeat_interval_input().value().parse::<u32>().unwrap();
        settings.save();
    }
}
