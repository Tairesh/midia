use roguemetry::Vec2;
use tetra::input::{Key, KeyModifier};
use tetra::window::WindowPosition;
use tetra::{Context, Event};

use crate::ui::ButtonBuilder;
use crate::{
    app::App,
    settings::Settings,
    ui::{
        Button, HasSize, Horizontal, Position, Press, SomeUISprites, SomeUISpritesMut, Stringify,
        TextInput, UiSprite, Vertical,
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
        let fullscreen_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[Alt+F] Fullscreen", app.assets.fonts.default.clone())
                .with_keys(vec![(Key::F, KeyModifier::Alt).into()])
                .with_position(Position::new(
                    Horizontal::CenterByLeft,
                    Vertical::TopByCenter,
                    Vec2::new(100.0, 200.0),
                ))
                .with_transition(Transition::CustomEvent(ButtonEvent::FullscreenMode as u8))
                .with_fixable(true)
                .with_pressed(settings.window.fullscreen)
                .build(),
        );
        let mut window_btn = Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text("[Alt+W] Window", app.assets.fonts.default.clone())
                .with_keys(vec![(Key::W, KeyModifier::Alt).into()])
                .with_position(Position::new(
                    Horizontal::CenterByRight,
                    Vertical::TopByCenter,
                    Vec2::new(98.0, 200.0),
                ))
                .with_transition(Transition::CustomEvent(ButtonEvent::WindowMode as u8))
                .with_fixable(true)
                .with_pressed(!settings.window.fullscreen)
                .build(),
        );
        let window_btn_size = window_btn.size(ctx);

        Self {
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg(&app.assets),
                title("Settings", &app.assets),
                fullscreen_btn,
                window_btn,
                label(
                    "Window mode:",
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByRight,
                        Vertical::TopByCenter,
                        Vec2::new(90.0 - window_btn_size.x, 200.0),
                    ),
                ),
                label(
                    "Repeat delay:",
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByRight,
                        Vertical::TopByCenter,
                        Vec2::new(90.0 - window_btn_size.x, 250.0),
                    ),
                ),
                icon_minus(
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByRight,
                        Vertical::TopByCenter,
                        Vec2::new(0.0, 250.0),
                    ),
                    ButtonEvent::RepeatIntervalMinus as u8,
                ),
                Box::new(TextInput::int(
                    settings.input.repeat_interval,
                    (1, 10000),
                    190.0,
                    app.assets.fonts.header.clone(),
                    Position::new(
                        Horizontal::CenterByLeft,
                        Vertical::TopByCenter,
                        Vec2::new(5.0, 250.0),
                    ),
                )),
                icon_plus(
                    &app.assets,
                    Position::new(
                        Horizontal::CenterByLeft,
                        Vertical::TopByCenter,
                        Vec2::new(200.0, 250.0),
                    ),
                    ButtonEvent::RepeatIntervalPlus as u8,
                ),
                back_btn(
                    Position::horizontal_center(Vertical::BottomByBottom, Vec2::new(0.0, -200.0)),
                    &app.assets,
                ),
            ],
        }
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
        easy_back(&event, self.get_update_context_state())
    }

    fn sprites(&self) -> SomeUISprites<'_> {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> Transition {
        let event = ButtonEvent::from(event);
        match event {
            ButtonEvent::FullscreenMode => {
                self.window_btn().unpress();
                if !tetra::window::is_fullscreen(ctx) {
                    Settings::instance().window.fullscreen = true;
                    if let Ok((width, height)) = tetra::window::get_current_monitor_size(ctx) {
                        tetra::window::set_size(ctx, width, height).ok();
                    }
                    tetra::window::set_fullscreen(ctx, true).ok();
                }
                Transition::None
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
                Transition::None
            }
            ButtonEvent::RepeatIntervalMinus | ButtonEvent::RepeatIntervalPlus => {
                let input = self.repeat_interval_input();
                if let Ok(mut value) = input.value().parse::<u32>() {
                    match event {
                        ButtonEvent::RepeatIntervalMinus => {
                            value -= 1;
                        }
                        ButtonEvent::RepeatIntervalPlus => {
                            value += 1;
                        }
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{value}"));
                    Settings::instance().input.repeat_interval = value;
                }
                Transition::None
            }
        }
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
