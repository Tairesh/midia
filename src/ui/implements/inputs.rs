use std::time::{Duration, Instant};

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::{
        mesh::{BorderRadii, Mesh, ShapeStyle},
        text::Text,
        Color, DrawParams, Rectangle,
    },
    input::{Key, KeyModifier, MouseButton},
    Context,
};

use super::super::{
    Disable, Draw, Focus, Hover, Position, Positionate, Press, Stringify, UiSprite, Update,
};
use crate::ui::UpdateContext;
use crate::{
    assets::PreparedFont,
    colors::Colors,
    input::{self, KeyWithMod},
    scenes::Transition,
};

enum ValueType {
    String { max_length: usize },
    Unsigned { min: u32, max: u32 },
}

struct InputValue {
    value_type: ValueType,
    text: Text,
    text_with_spaces: Text,
}

impl InputValue {
    pub fn new(value_type: ValueType, value: impl Into<String>, font: PreparedFont) -> Self {
        let value = value.into();
        Self {
            value_type,
            text_with_spaces: Text::new(value.replace(' ', "_"), font.font.clone()),
            text: Text::new(value, font.font),
        }
    }

    pub fn text(&mut self) -> &mut Text {
        &mut self.text
    }

    pub fn text_with_spaces(&mut self) -> &mut Text {
        &mut self.text_with_spaces
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        let value = value.into();
        self.text.set_content(value.clone());
        self.text_with_spaces.set_content(value.replace(' ', "_"));
        self.validate();
    }

    pub fn value(&self) -> String {
        self.text.content().to_string()
    }

    pub fn validate(&mut self) {
        if let ValueType::Unsigned { min, max } = self.value_type {
            let val = self.text.content().parse::<u32>().unwrap_or(min);
            if val < min || val > max {
                self.set_value(val.clamp(min, max).to_string());
            }
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();
        self.text_with_spaces.pop();
    }

    pub fn try_push_str(&mut self, s: &str) -> bool {
        let allow = match self.value_type {
            ValueType::String { .. } => true,
            ValueType::Unsigned { .. } => s.parse::<u32>().is_ok(),
        };
        if !allow {
            return false;
        }

        self.text.push_str(s);
        self.text_with_spaces.push_str(s.replace(' ', "_").as_str());
        if let ValueType::String { max_length } = self.value_type {
            if self.text.content().len() > max_length {
                self.text
                    .set_content(String::from(&self.text.content()[..max_length]));
                self.text_with_spaces
                    .set_content(String::from(&self.text_with_spaces.content()[..max_length]));
            }
        }

        true
    }
}

struct InputGeometry {
    pub width: f32,
    pub line_height: f32,
    pub rect: Option<Rect>,
}

struct InputBlink {
    pub state: bool,
    pub last_blinked: Instant,
}

impl InputBlink {
    pub fn toggle(&mut self) {
        self.state = !self.state;
        self.last_blinked = Instant::now();
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
        self.last_blinked = Instant::now();
    }

    pub fn its_time(&self) -> bool {
        self.last_blinked.elapsed() > Duration::new(0, 500_000_000)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputState {
    Default,
    Focused,
    Disabled,
    Hovered,
    Danger,
}

pub struct TextInput {
    value: InputValue,
    position: Position,
    geometry: InputGeometry,
    state: InputState,
    blink: InputBlink,
    visible: bool,
    bg: Option<Mesh>,
    border: Option<Mesh>,
    cursor: Option<Mesh>,
}

impl TextInput {
    pub fn new<C: Into<String>>(
        value: C,
        width: f32,
        font: PreparedFont,
        position: Position,
    ) -> Self {
        let value = value.into();
        Self {
            geometry: InputGeometry {
                width,
                line_height: font.line_height,
                rect: None,
            },
            value: InputValue::new(ValueType::String { max_length: 32 }, value, font),
            blink: InputBlink {
                state: false,
                last_blinked: Instant::now(),
            },
            state: InputState::Default,
            visible: true,
            bg: None,
            border: None,
            cursor: None,
            position,
        }
    }

    pub fn int(
        value: u32,
        clamps: (u32, u32),
        width: f32,
        font: PreparedFont,
        position: Position,
    ) -> Self {
        let mut s = Self::new(value.to_string(), width, font, position);
        s.value.value_type = ValueType::Unsigned {
            min: clamps.0,
            max: clamps.1,
        };
        s
    }

    fn border_color(&self) -> Color {
        match self.state {
            InputState::Danger => Colors::DARK_RED,
            InputState::Disabled => Colors::DARK_GRAY,
            InputState::Focused => Colors::DARK_GREEN,
            _ => Colors::DARK_BROWN,
        }
    }

    fn bg_color(&self) -> Option<Color> {
        match self.state {
            InputState::Danger => Some(Colors::RED.with_alpha(0.8)),
            InputState::Disabled => Some(Colors::DARK_GRAY.with_alpha(0.8)),
            InputState::Focused => Some(Colors::DARK_GREEN.with_alpha(0.8)),
            InputState::Hovered => Some(Colors::DARK_BROWN.with_alpha(0.2)),
            InputState::Default => None,
        }
    }

    fn text_color(&self) -> Color {
        match self.state {
            InputState::Disabled => Colors::WHITE,
            InputState::Focused => Colors::LIGHT_YELLOW,
            _ => Colors::DARK_BROWN,
        }
    }

    pub fn set_danger(&mut self, danger: bool) {
        self.state = if danger {
            InputState::Danger
        } else {
            InputState::Default
        };
    }

    pub fn is_danger(&self) -> bool {
        matches!(self.state, InputState::Danger)
    }
}

impl Draw for TextInput {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.geometry.rect.unwrap();
        if let Some(bg_color) = self.bg_color() {
            self.bg.as_ref().unwrap().draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(rect.x, rect.y))
                    .color(bg_color),
            );
        }
        self.border.as_ref().unwrap().draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .color(self.border_color()),
        );
        let text_width = self
            .value
            .text_with_spaces()
            .get_bounds(ctx)
            .map_or(-1.0, |r| r.width + 3.0);
        let y = (rect.y + rect.h / 2.0 - self.geometry.line_height / 2.0 + 2.0).round();
        let text_pos = if self.state == InputState::Focused {
            Vec2::new(rect.x + 7.0, y)
        } else {
            Vec2::new(rect.x + rect.w / 2.0 - text_width / 2.0, y)
        };
        // TODO: horizontal scroll if text width is bigger than sprite width
        let color = self.text_color();
        self.value
            .text()
            .draw(ctx, DrawParams::new().position(text_pos).color(color));
        if self.blink.state && self.state == InputState::Focused {
            self.cursor.as_ref().unwrap().draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        rect.x + text_width + 10.0,
                        rect.y + rect.h / 2.0 - self.geometry.line_height.midpoint(8.0),
                    ))
                    .color(color),
            );
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for TextInput {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let (w, h) = (self.geometry.width, self.geometry.line_height + 16.0);
        self.bg = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, w, h),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        self.border = Some(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Stroke(2.0),
                Rectangle::new(0.0, 0.0, w, h),
                BorderRadii::new(5.0),
            )
            .unwrap(),
        );
        self.cursor = Some(
            Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, 10.0, h - 8.0),
            )
            .unwrap(),
        );
        Vec2::new(w, h)
    }

    fn rect(&self) -> Rect {
        self.geometry.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.geometry.rect = Some(rect);
    }
}

impl Update for TextInput {
    fn update(&mut self, ctx: UpdateContext) -> Option<Transition> {
        let hovered = ctx.is_hovered(self.rect());

        if self.state == InputState::Default && hovered {
            self.on_hovered();
        } else if self.state == InputState::Hovered && !hovered {
            self.off_hovered();
        } else if self.state == InputState::Focused {
            if input::is_mouse_button_pressed(ctx.ctx, MouseButton::Left) && !hovered {
                self.off_pressed();
            }
            if self.blink.its_time() {
                self.blink.toggle();
            }
            if input::is_key_pressed(ctx.ctx, Key::Backspace)
                && !self.value.text().content().is_empty()
            {
                self.value.backspace();
            }
            if input::is_key_pressed(ctx.ctx, Key::Enter)
                || input::is_key_pressed(ctx.ctx, Key::Escape)
            {
                self.set_focused(false);
            }
            if let Some(text_input) = input::get_text_input(ctx.ctx) {
                self.value.try_push_str(text_input);
            }
            if input::is_key_with_mod_pressed(ctx.ctx, KeyWithMod::ctrl(Key::V))
                || input::is_key_with_mod_pressed(ctx.ctx, KeyWithMod::shift(Key::Insert))
            {
                let clipboard: String = input::get_clipboard_text(ctx.ctx)
                    .unwrap()
                    .chars()
                    .map(|c| if c == '\n' { ' ' } else { c })
                    .collect();
                self.value.try_push_str(clipboard.as_str());
            }
        } else if input::is_mouse_button_pressed(ctx.ctx, MouseButton::Left)
            && hovered
            && self.state != InputState::Disabled
        {
            self.on_pressed();
        }
        None
    }
}

impl Disable for TextInput {
    fn disabled(&self) -> bool {
        self.state == InputState::Disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled {
            self.state = InputState::Disabled;
        } else {
            self.state = InputState::Default;
        }
    }
}

impl Stringify for TextInput {
    fn value(&self) -> String {
        self.value.value()
    }

    fn set_value<C: Into<String>>(&mut self, value: C) {
        self.value.set_value(value);
        if self.state == InputState::Danger {
            self.state = InputState::Default;
        }
    }
}

impl Hover for TextInput {
    fn on_hovered(&mut self) {
        self.state = InputState::Hovered;
    }

    fn off_hovered(&mut self) {
        self.state = InputState::Default;
    }

    fn hovered(&self) -> bool {
        self.state == InputState::Hovered
    }
}

impl Press for TextInput {
    fn on_pressed(&mut self) {
        self.state = InputState::Focused;
        self.blink.set_state(true);
    }

    fn off_pressed(&mut self) {
        self.unpress();
    }

    fn unpress(&mut self) {
        self.state = InputState::Default;
        self.blink.set_state(false);
    }

    fn pressed(&self) -> bool {
        self.state == InputState::Focused
    }
}

impl Focus for TextInput {
    fn focused(&self) -> bool {
        self.state == InputState::Focused
    }

    fn set_focused(&mut self, focused: bool) {
        if focused {
            self.on_pressed();
        } else {
            self.off_pressed();
        }
    }
}

impl UiSprite for TextInput {
    fn as_text_input(&mut self) -> Option<&mut TextInput> {
        Some(self)
    }
}
