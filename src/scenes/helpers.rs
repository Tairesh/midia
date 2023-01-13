use tetra::{
    graphics::Color,
    input::{Key, KeyModifier, MouseButton},
    Context, Event,
};

use crate::{
    assets::Assets,
    colors::Colors,
    ui::{Button, Horizontal, Image, Label, Position, Positionate, TextInput, Vertical},
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
    Box::new(Image::repeat(assets.images.bg.clone()))
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
    // TODO: positionate in center
    let y = Vertical::AtWindowBottomByBottom { offset: -50.0 };
    let mut randomize_btn = randomize_btn(
        assets,
        Position {
            x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
            y,
        },
        randomize,
    );
    let randomize_btn_size = randomize_btn.calc_size(ctx);
    let back_btn = back_btn(
        Position {
            x: Horizontal::AtWindowCenterByRight {
                offset: -randomize_btn_size.x / 2.0 - 2.0,
            },
            y,
        },
        assets,
    );
    let next_btn = next_btn(
        assets,
        Position {
            x: Horizontal::AtWindowCenterByLeft {
                offset: randomize_btn_size.x / 2.0 + 2.0,
            },
            y,
        },
        next,
        next_text,
    );
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
