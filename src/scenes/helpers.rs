use tetra::{
    graphics::Color,
    input::{Key, KeyModifier, MouseButton},
    Context, Event,
};

use crate::{
    assets::Assets,
    colors::Colors,
    ui::{Button, Image, Label, Position, Positionate, TextInput, Vertical},
};

use super::{SomeTransitions, Transition};

pub(crate) fn easy_back(event: &Event, focused: bool) -> SomeTransitions {
    if focused {
        return None;
    }
    match event {
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        }
        | Event::KeyPressed {
            key: Key::Backspace,
        } => Some(vec![Transition::Pop]),
        _ => None,
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
        Position::horizontal_center(0.0, Vertical::ByTop { y: 90.0 }),
    ))
}

pub(crate) fn subtitle(subtitle: impl Into<String>, assets: &Assets) -> Box<Label> {
    Box::new(Label::new(
        subtitle,
        assets.fonts.subtitle.clone(),
        Colors::DARK_BROWN,
        Position::horizontal_center(0.0, Vertical::ByTop { y: 130.0 }),
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
    Box::new(Button::text(
        vec![Key::Escape.into()],
        "[Esc] Back",
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::Pop,
    ))
}

pub(crate) fn next_btn(
    assets: &Assets,
    position: Position,
    custom_event: u8,
    text: &str,
) -> Box<Button> {
    Box::new(Button::text(
        vec![(Key::Enter, KeyModifier::Alt).into()],
        format!("[Alt+Enter] {text}"),
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
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
    Box::new(Button::text(
        vec![
            Key::NumPadMultiply.into(),
            (Key::Num8, KeyModifier::Shift).into(),
        ],
        "[*] Randomize",
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
}

pub(crate) fn back_randomize_next(
    assets: &Assets,
    ctx: &mut Context,
    randomize: u8,
    next: u8,
    next_text: &str,
) -> (Box<Button>, Box<Button>, Box<Button>) {
    let mut randomize_btn = randomize_btn(assets, Position::center(), randomize);
    let randomize_btn_size = randomize_btn.calc_size(ctx);

    let mut back_btn = back_btn(Position::center(), assets);
    let back_btn_size = back_btn.calc_size(ctx);

    let mut next_btn = next_btn(assets, Position::center(), next, next_text);
    let next_btn_size = next_btn.calc_size(ctx);

    let total_width = randomize_btn_size.x + back_btn_size.x + next_btn_size.x + 4.0;
    let y = Vertical::AtWindowBottomByBottom { offset: -50.0 };
    // positionate them in center
    randomize_btn.set_position(Position::horizontal_center(
        -total_width / 2.0 + randomize_btn_size.x / 2.0,
        y,
    ));
    back_btn.set_position(Position::horizontal_center(
        -total_width / 2.0 + randomize_btn_size.x + back_btn_size.x / 2.0 + 2.0,
        y,
    ));
    next_btn.set_position(Position::horizontal_center(
        -total_width / 2.0 + randomize_btn_size.x + back_btn_size.x + next_btn_size.x / 2.0 + 4.0,
        y,
    ));

    (back_btn, randomize_btn, next_btn)
}

pub(crate) fn icon_left(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(Button::icon(
        vec![],
        "lt",
        assets.tileset.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
}

pub(crate) fn icon_right(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(Button::icon(
        vec![],
        "mt",
        assets.tileset.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
}

pub(crate) fn icon_plus(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(Button::icon(
        vec![],
        "plus",
        assets.tileset.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
}

pub(crate) fn icon_minus(assets: &Assets, position: Position, custom_event: u8) -> Box<Button> {
    Box::new(Button::icon(
        vec![],
        "minus",
        assets.tileset.clone(),
        assets.button.clone(),
        position,
        Transition::CustomEvent(custom_event),
    ))
}
