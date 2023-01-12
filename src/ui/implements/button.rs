#![allow(dead_code)]

use std::rc::Rc;

use geometry::{Rect, Vec2};
use tetra::{
    graphics::{text::Text, DrawParams},
    Context,
};

use crate::{
    assets::{Button as ButtonAsset, PreparedFont, Tileset},
    colors::Colors,
    input::{self, KeyWithMod, MouseButton},
    scenes::Transition,
};

use super::super::{Disable, Draw, Focus, Hover, Position, Positionate, Press, UiSprite, Update};

enum ButtonContent {
    Text(Text, f32),
    Icon {
        name: &'static str,
        scale: Vec2,
        tileset: Rc<Tileset>,
    },
    Empty(Vec2),
}

impl ButtonContent {
    pub const fn offset_x(&self) -> f32 {
        match self {
            ButtonContent::Text(..) => 30.0,
            ButtonContent::Empty(..) => 0.0,
            ButtonContent::Icon { .. } => 10.0,
        }
    }
}

enum ButtonState {
    Default,
    Pressed,
    Hovered,
    PressedAndHovered,
    Disabled,
}

impl ButtonState {
    pub fn on_hover(&mut self) {
        (*self) = match self {
            Self::Default | Self::Hovered | Self::PressedAndHovered => Self::Hovered,
            Self::Pressed => Self::Pressed,
            Self::Disabled => Self::Disabled,
        }
    }

    pub fn off_hover(&mut self) {
        (*self) = match self {
            Self::Default | Self::Hovered => Self::Default,
            Self::Pressed | Self::PressedAndHovered => Self::Pressed,
            Self::Disabled => Self::Disabled,
        }
    }

    pub fn on_press(&mut self) {
        (*self) = match self {
            Self::Default | Self::Hovered | Self::Pressed => Self::Pressed,
            Self::PressedAndHovered => Self::PressedAndHovered,
            Self::Disabled => Self::Disabled,
        }
    }

    pub fn off_press(&mut self) {
        (*self) = match self {
            Self::Default | Self::Hovered | Self::Pressed => Self::Default,
            Self::PressedAndHovered => Self::Hovered,
            Self::Disabled => Self::Disabled,
        }
    }
}

pub struct Button {
    keys: Vec<KeyWithMod>,
    content: ButtonContent,
    on_click: Transition,
    position: Position,
    asset: Rc<ButtonAsset>,
    scale: Vec2,
    rect: Option<Rect>,
    state: ButtonState,
    fixable: bool,
    visible: bool,
}

impl Button {
    fn new(
        keys: Vec<KeyWithMod>,
        content: ButtonContent,
        asset: Rc<ButtonAsset>,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            keys,
            content,
            on_click,
            position,
            asset,
            scale: Vec2::new(3.0, 3.0),
            rect: None,
            state: ButtonState::Default,
            fixable: false,
            visible: true,
        }
    }

    pub fn text<S>(
        keys: Vec<KeyWithMod>,
        text: S,
        font: PreparedFont,
        asset: Rc<ButtonAsset>,
        position: Position,
        on_click: Transition,
    ) -> Self
    where
        S: Into<String>,
    {
        Self::new(
            keys,
            ButtonContent::Text(Text::new(text, font.font), font.line_height),
            asset,
            position,
            on_click,
        )
    }

    pub fn empty(
        keys: Vec<KeyWithMod>,
        asset: Rc<ButtonAsset>,
        size: Vec2,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(keys, ButtonContent::Empty(size), asset, position, on_click)
    }

    pub fn fixed(
        keys: Vec<KeyWithMod>,
        text: &str,
        font: PreparedFont,
        asset: Rc<ButtonAsset>,
        state: bool,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            fixable: true,
            state: if state {
                ButtonState::Pressed
            } else {
                ButtonState::Default
            },
            ..Self::text(keys, text, font, asset, position, on_click)
        }
    }

    pub fn icon(
        keys: Vec<KeyWithMod>,
        name: &'static str,
        tileset: Rc<Tileset>,
        asset: Rc<ButtonAsset>,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::Icon {
                name,
                scale: Vec2::new(3.0, 3.0),
                tileset,
            },
            asset,
            position,
            on_click,
        )
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.set_disabled(val);
        self
    }

    pub fn custom_event(&self) -> Option<u8> {
        if let Transition::CustomEvent(s) = self.on_click {
            Some(s)
        } else {
            None
        }
    }

    fn content_size(&mut self, ctx: &mut Context) -> Vec2 {
        match &mut self.content {
            ButtonContent::Text(text, height) => text
                .get_bounds(ctx)
                .map(|b| Vec2::new(b.width, *height))
                .unwrap(),
            ButtonContent::Empty(size) => *size,
            ButtonContent::Icon { scale, tileset, .. } => Vec2::new(
                tileset.tile_size as f32 * scale.x,
                tileset.tile_size as f32 * scale.y,
            ),
        }
    }
}

impl Draw for Button {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = Vec2::new(rect.x, rect.y);
        let content_size = self.content_size(ctx);

        let config = match self.state {
            ButtonState::Default => &self.asset.default,
            ButtonState::Pressed | ButtonState::PressedAndHovered => &self.asset.pressed,
            ButtonState::Hovered => &self.asset.hovered,
            ButtonState::Disabled => &self.asset.disabled,
        };
        self.asset.texture.draw_nine_slice(
            ctx,
            config,
            rect.w / self.scale.x,
            rect.h / self.scale.y,
            DrawParams::new().position(vec).scale(self.scale),
        );

        vec += Vec2::new(rect.w, rect.h) / 2.0 - content_size / 2.0;
        if !self.pressed() {
            vec.y -= 2.0;
        }
        match &mut self.content {
            ButtonContent::Text(text, _) => {
                text.draw(
                    ctx,
                    DrawParams::new().position(vec).color(Colors::LIGHT_YELLOW),
                );
            }
            ButtonContent::Icon {
                name,
                scale,
                tileset,
            } => {
                vec.y -= 1.0;
                tileset.draw_region(ctx, name, DrawParams::new().position(vec).scale(*scale));
            }
            ButtonContent::Empty(_) => {}
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Button {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let content_size = self.content_size(ctx);
        let offset_x = self.content.offset_x();
        Vec2::new(content_size.x + offset_x, 42.0)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Button {
    fn update(&mut self, ctx: &mut Context, focused: bool, blocked: &[Rect]) -> Option<Transition> {
        if self.disabled() {
            return None;
        }
        if !self.keys.is_empty() && !focused {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for kwm in self.keys.iter().copied() {
                if input::is_key_with_mod_pressed(ctx, kwm) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, kwm.key) && self.pressed() {
                    off_pressed = true;
                }
            }
            if on_pressed {
                self.on_pressed();
            } else if off_pressed {
                self.off_pressed();
                return Some(self.on_click.clone());
            }
        }
        let mouse = input::get_mouse_position(ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        if collides && blocked.iter().any(|r| r.contains_point(mouse)) {
            return None;
        }
        if !self.hovered() && collides {
            self.on_hovered();
        } else if self.hovered() && !collides {
            self.off_hovered();
        }
        if collides && !self.pressed() && input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.on_pressed();
        } else if self.pressed() && input::is_mouse_button_released(ctx, MouseButton::Left) {
            self.off_pressed();
            if collides {
                return Some(self.on_click.clone());
            }
        }
        None
    }
}

impl Disable for Button {
    fn disabled(&self) -> bool {
        matches!(self.state, ButtonState::Disabled)
    }

    fn set_disabled(&mut self, value: bool) {
        if value && !self.disabled() {
            self.state = ButtonState::Disabled;
        } else if !value && self.disabled() {
            self.state = ButtonState::Default;
        }
    }
}

impl Hover for Button {
    fn on_hovered(&mut self) {
        self.state.on_hover();
    }

    fn off_hovered(&mut self) {
        self.state.off_hover();
    }

    fn hovered(&self) -> bool {
        matches!(
            self.state,
            ButtonState::Hovered | ButtonState::PressedAndHovered
        )
    }
}

impl Press for Button {
    fn on_pressed(&mut self) {
        self.state.on_press();
    }

    fn off_pressed(&mut self) {
        if !self.fixable {
            self.unpress();
        }
    }

    fn unpress(&mut self) {
        self.state.off_press();
    }

    fn pressed(&self) -> bool {
        matches!(
            self.state,
            ButtonState::Pressed | ButtonState::PressedAndHovered
        )
    }
}

impl Focus for Button {}

impl UiSprite for Button {
    fn as_button(&mut self) -> Option<&mut Button> {
        Some(self)
    }
}
