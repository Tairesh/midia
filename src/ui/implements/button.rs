#![allow(dead_code)]

use std::rc::Rc;

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::{text::Text, Color, DrawParams},
    Context,
};

use super::super::{Disable, Draw, Focus, Hover, Position, Positionate, Press, UiSprite, Update};
use crate::assets::Sprite;
use crate::ui::UpdateContext;
use crate::{
    assets::{Button as ButtonAsset, PreparedFont, Tileset},
    colors::Colors,
    input::{self, KeyWithMod, MouseButton},
    scenes::Transition,
};

enum ButtonContent {
    Text(Text, f32),
    Icon {
        sprite: Sprite,
        scale: Vec2,
        color: Option<Color>,
        tileset: Rc<Tileset>,
    },
    Empty(Vec2),
}

impl ButtonContent {
    pub const fn offset(&self) -> f32 {
        match self {
            ButtonContent::Text(..) => 30.0,
            ButtonContent::Empty(..) => 0.0,
            ButtonContent::Icon { .. } => 10.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
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
    pub fn update_icon(
        &mut self,
        sprite: impl Into<Sprite>,
        scale: Vec2,
        color: Option<Color>,
        ctx: &mut Context,
        window_size: (i32, i32),
    ) {
        if let ButtonContent::Icon { tileset, .. } = &self.content {
            self.content = ButtonContent::Icon {
                sprite: sprite.into(),
                scale,
                color,
                tileset: tileset.clone(),
            }
        }
        self.positionate(ctx, window_size);
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
            vec.y -= 1.0;
        }
        match &mut self.content {
            ButtonContent::Text(text, _) => {
                let color = if self.state == ButtonState::Disabled {
                    Colors::LIGHT_GRAY
                } else {
                    Colors::LIGHT_YELLOW
                };
                text.draw(ctx, DrawParams::new().position(vec).color(color));
            }
            ButtonContent::Icon {
                sprite,
                scale,
                tileset,
                color,
            } => {
                vec.y -= 1.0;
                let mut params = DrawParams::new().position(vec).scale(*scale);
                if let Some(color) = color {
                    params = params.color(*color);
                }
                if self.state == ButtonState::Disabled {
                    params = params.color(Colors::GRAY);
                }
                tileset.draw_sprite(ctx, *sprite, params);
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
        let offset_x = self.content.offset();
        Vec2::new(
            content_size.x + offset_x,
            if let ButtonContent::Icon { sprite, scale, .. } = &self.content {
                Tileset::get_size(*sprite).y * scale.y + self.content.offset()
            } else {
                42.0
            },
        )
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Button {
    fn update(&mut self, ctx: UpdateContext) -> Option<Transition> {
        if self.disabled() {
            return None;
        }
        if !self.keys.is_empty() && !ctx.is_focused() {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for kwm in self.keys.iter().copied() {
                if input::is_key_with_mod_pressed(ctx.ctx, kwm) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx.ctx, kwm.key) && self.pressed() {
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
        let hovered = ctx.is_hovered(self.rect?);
        if !self.hovered() && hovered {
            self.on_hovered();
        } else if self.hovered() && !hovered {
            self.off_hovered();
        }
        if hovered && !self.pressed() && input::is_mouse_button_pressed(ctx.ctx, MouseButton::Left)
        {
            self.on_pressed();
        } else if self.pressed() && input::is_mouse_button_released(ctx.ctx, MouseButton::Left) {
            self.off_pressed();
            if hovered {
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

pub struct ButtonBuilder {
    keys: Vec<KeyWithMod>,
    content: Option<ButtonContent>,
    on_click: Option<Transition>,
    position: Option<Position>,
    asset: Rc<ButtonAsset>,
    scale: Vec2,
    state: ButtonState,
    fixable: bool,
    visible: bool,
    pressed: bool,
}

impl ButtonBuilder {
    pub fn new(asset: Rc<ButtonAsset>) -> Self {
        Self {
            keys: Vec::new(),
            content: None,
            on_click: None,
            position: None,
            asset,
            scale: Vec2::new(3.0, 3.0),
            state: ButtonState::Default,
            fixable: false,
            visible: true,
            pressed: false,
        }
    }

    pub fn with_keys(mut self, keys: impl Into<Vec<KeyWithMod>>) -> Self {
        self.keys = keys.into();

        self
    }

    pub fn with_text(mut self, text: impl Into<String>, font: PreparedFont) -> Self {
        self.content = Some(ButtonContent::Text(
            Text::new(text, font.font),
            font.line_height,
        ));

        self
    }

    pub fn with_empty(mut self, size: Vec2) -> Self {
        self.content = Some(ButtonContent::Empty(size));

        self
    }

    pub fn with_icon(
        mut self,
        sprite: impl Into<Sprite>,
        scale: Vec2,
        color: Option<Color>,
        tileset: Rc<Tileset>,
    ) -> Self {
        self.content = Some(ButtonContent::Icon {
            sprite: sprite.into(),
            scale,
            color,
            tileset,
        });

        self
    }

    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.on_click = Some(transition);

        self
    }

    pub fn with_position(mut self, position: Position) -> Self {
        self.position = Some(position);

        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;

        self
    }

    pub fn with_state(mut self, state: ButtonState) -> Self {
        self.state = state;

        self
    }

    pub fn with_fixable(mut self, fixable: bool) -> Self {
        self.fixable = fixable;

        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;

        self
    }

    pub fn with_pressed(mut self, pressed: bool) -> Self {
        self.state = if pressed {
            ButtonState::Pressed
        } else {
            ButtonState::Default
        };

        self
    }

    pub fn build(self) -> Button {
        assert!(
            !(self.content.is_none() || self.on_click.is_none() || self.position.is_none()),
            "Invalid button config"
        );

        Button {
            keys: self.keys,
            content: self.content.unwrap(),
            on_click: self.on_click.unwrap(),
            position: self.position.unwrap(),
            asset: self.asset,
            scale: self.scale,
            rect: None,
            state: self.state,
            fixable: self.fixable,
            visible: self.visible,
        }
    }
}
