use roguemetry::Vec2;
use tetra::{
    graphics::Color,
    input::{Key, KeyModifier, MouseButton},
    Context, Event,
};

use crate::ui::{ButtonBuilder, UiSprite};
use crate::{
    assets::{Assets, Sprite},
    colors::Colors,
    ui::{Button, HasLayout, HasSize, Image, Label, Position, TextInput, Vertical},
};

use super::Transition;

pub const UI_ICONS_SCALE: Vec2 = Vec2::new(3.0, 3.0);

pub(crate) fn window_size(ctx: &mut Context) -> Vec2 {
    let window_size = tetra::window::get_size(ctx);
    Vec2::new(window_size.0 as f32, window_size.1 as f32)
}

pub(crate) fn easy_back(event: &Event) -> Transition {
    match event {
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        }
        | Event::KeyPressed {
            key: Key::Backspace,
        } => Transition::Pop,
        _ => Transition::None,
    }
}

pub(crate) fn bg(assets: &Assets) -> Box<Image> {
    Box::new(Image::auto_size(assets.images.bg.clone()))
}

pub(crate) fn title(title: impl Into<String>, assets: &Assets) -> Box<Label> {
    Box::new(Label::new(
        title,
        assets.fonts.title.clone(),
        Colors::DARK_BROWN,
        Position::horizontal_center(Vertical::TopByTop, Vec2::new(0.0, 50.0)),
    ))
}

pub(crate) fn subtitle(subtitle: impl Into<String>, assets: &Assets) -> Box<Label> {
    Box::new(Label::new(
        subtitle,
        assets.fonts.subtitle.clone(),
        Colors::DARK_BROWN,
        Position::horizontal_center(Vertical::TopByTop, Vec2::new(0.0, 95.0)),
    ))
}

pub(crate) fn label(text: impl Into<String>, assets: &Assets, position: Position) -> Box<Label> {
    Box::new(Label::new(
        text,
        assets.fonts.header.clone(),
        Colors::DARK_BROWN,
        position,
    ))
}

pub(crate) fn colored_label(
    text: impl Into<String>,
    assets: &Assets,
    position: Position,
    color: Color,
) -> Box<Label> {
    Box::new(Label::new(
        text,
        assets.fonts.header.clone(),
        color,
        position,
    ))
}

pub(crate) fn decorative_label(
    text: impl Into<String>,
    assets: &Assets,
    position: Position,
    color: Color,
) -> Box<Label> {
    Box::new(Label::new(
        text,
        assets.fonts.subtitle.clone(),
        color,
        position,
    ))
}

pub(crate) fn error_label(
    text: impl Into<String>,
    assets: &Assets,
    position: Position,
) -> Box<Label> {
    Box::new(Label::hidden(
        text,
        assets.fonts.default.clone(),
        Colors::RED,
        position,
    ))
}

pub(crate) fn back_btn(position: Position, assets: &Assets) -> Box<Button> {
    Box::new(
        ButtonBuilder::new(assets.button.clone())
            .with_text("[Esc] Back", assets.fonts.default.clone())
            .with_keys(vec![Key::Escape.into()])
            .with_position(position)
            .with_transition(Transition::Pop)
            .build(),
    )
}

pub(crate) fn next_btn(
    assets: &Assets,
    position: Position,
    custom_event: u8,
    text: &str,
) -> Box<Button> {
    Box::new(
        ButtonBuilder::new(assets.button.clone())
            .with_text(format!("[Alt+Enter] {text}"), assets.fonts.default.clone())
            .with_keys(vec![(Key::Enter, KeyModifier::Alt).into()])
            .with_position(position)
            .with_transition(Transition::CustomEvent(custom_event))
            .build(),
    )
}

pub(crate) fn text_input(
    value: impl Into<String>,
    width: f32,
    assets: &Assets,
    position: Position,
) -> Box<TextInput> {
    Box::new(TextInput::new(
        value,
        width,
        assets.fonts.header.clone(),
        position,
    ))
}

pub(crate) fn randomize_btn(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(
        ButtonBuilder::new(assets.button.clone())
            .with_text("[*] Randomize", assets.fonts.default.clone())
            .with_keys(vec![
                Key::NumPadMultiply.into(),
                (Key::Num8, KeyModifier::Shift).into(),
            ])
            .with_position(position)
            .with_transition(Transition::CustomEvent(custom_event))
            .build(),
    )
}

pub(crate) fn back_randomize_next(
    assets: &Assets,
    ctx: &mut Context,
    randomize: u8,
    next: u8,
    next_text: &str,
) -> [Box<dyn UiSprite>; 3] {
    let mut randomize_btn = randomize_btn(assets, Position::zero(), randomize);
    let randomize_btn_size = randomize_btn.size(ctx);

    let mut back_btn = back_btn(Position::zero(), assets);
    let back_btn_size = back_btn.size(ctx);

    let mut next_btn = next_btn(assets, Position::zero(), next, next_text);
    let next_btn_size = next_btn.size(ctx);

    let total_width = randomize_btn_size.x + back_btn_size.x + next_btn_size.x + 4.0;

    // positionate them in center
    back_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(-total_width / 2.0 + back_btn_size.x / 2.0, -50.0),
        ));
    randomize_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(
                -total_width / 2.0 + back_btn_size.x + randomize_btn_size.x / 2.0 + 2.0,
                -50.0,
            ),
        ));
    next_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(
                -total_width / 2.0
                    + back_btn_size.x
                    + randomize_btn_size.x
                    + next_btn_size.x / 2.0
                    + 4.0,
                -50.0,
            ),
        ));

    [back_btn, randomize_btn, next_btn]
}

pub(crate) fn reset_btn(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(
        ButtonBuilder::new(assets.button.clone())
            .with_text("[Ctrl+R] Reset", assets.fonts.default.clone())
            .with_keys(vec![(Key::R, KeyModifier::Ctrl).into()])
            .with_position(position)
            .with_transition(Transition::CustomEvent(custom_event))
            .build(),
    )
}

pub(crate) fn back_randomize_reset_next(
    assets: &Assets,
    ctx: &mut Context,
    randomize: u8,
    reset: u8,
    next: u8,
    next_text: &str,
) -> [Box<dyn UiSprite>; 4] {
    let mut randomize_btn = randomize_btn(assets, Position::zero(), randomize);
    let randomize_btn_size = randomize_btn.size(ctx);

    let mut back_btn = back_btn(Position::zero(), assets);
    let back_btn_size = back_btn.size(ctx);

    let mut reset_btn = reset_btn(assets, Position::zero(), reset);
    let reset_btn_size = reset_btn.size(ctx);

    let mut next_btn = next_btn(assets, Position::zero(), next, next_text);
    let next_btn_size = next_btn.size(ctx);

    let total_width =
        randomize_btn_size.x + back_btn_size.x + reset_btn_size.x + next_btn_size.x + 6.0;

    // positionate them in center
    back_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(-total_width / 2.0 + back_btn_size.x / 2.0, -50.0),
        ));
    randomize_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(
                -total_width / 2.0 + back_btn_size.x + randomize_btn_size.x / 2.0 + 2.0,
                -50.0,
            ),
        ));
    reset_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(
                -total_width / 2.0
                    + back_btn_size.x
                    + randomize_btn_size.x
                    + reset_btn_size.x / 2.0
                    + 4.0,
                -50.0,
            ),
        ));
    next_btn
        .layout_mut()
        .set_position(Position::horizontal_center(
            Vertical::BottomByBottom,
            Vec2::new(
                -total_width / 2.0
                    + back_btn_size.x
                    + randomize_btn_size.x
                    + reset_btn_size.x
                    + next_btn_size.x / 2.0
                    + 6.0,
                -50.0,
            ),
        ));

    [back_btn, randomize_btn, reset_btn, next_btn]
}

pub fn icon_button(
    assets: &Assets,
    sprite: impl Into<Sprite>,
    position: Position,
    custom_event: u8,
) -> Box<Button> {
    Box::new(
        ButtonBuilder::new(assets.button.clone())
            .with_icon(sprite, UI_ICONS_SCALE, None, assets.tileset.clone())
            .with_position(position)
            .with_transition(Transition::CustomEvent(custom_event))
            .build(),
    )
}

pub(crate) fn icon_left(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    icon_button(assets, Sprite::LessThan, position, custom_event)
}

pub(crate) fn icon_right(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    icon_button(assets, Sprite::MoreThan, position, custom_event)
}

pub(crate) fn icon_plus(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    icon_button(assets, Sprite::Plus, position, custom_event)
}

pub(crate) fn icon_minus(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    icon_button(assets, Sprite::Minus, position, custom_event)
}
