#![allow(dead_code)]

use crate::scenes::Transition;
use crate::ui::JustMesh;
use roguemetry::{Rect, Vec2};
use tetra::graphics::Color;
use tetra::input::MouseButton;
use tetra::{input, Context};

use super::{Alert, Button, Label, Position, TextInput, TilesetSprite};

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub trait Sizeable {
    fn calc_size(&mut self, ctx: &mut Context) -> Vec2;
}

// TODO: make Layout struct to handle position and rect, and trait with only layout_mut() method
pub trait Positionable: Sizeable {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
    fn update_position(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        let owner_size = self.calc_size(ctx);
        let left_top = self.position().calc(owner_size, window_size);
        let rect = Rect::new(left_top.x, left_top.y, owner_size.x, owner_size.y);
        self.set_rect(rect);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UpdateContextState {
    #[default]
    Normal,
    /// There is some focused sprite on the scene (e.g. text input intercepting all text input, so buttons shouldn't react to their hotkeys)
    Focused,
}

pub struct UpdateContext<'a> {
    pub ctx: &'a mut Context,
    /// Rectangles of UI elements above current one, blocking mouse interaction
    pub blocked: &'a [Rect],
    pub state: UpdateContextState,
}

impl<'a> UpdateContext<'a> {
    pub fn new(ctx: &'a mut Context, blocked: &'a [Rect], state: UpdateContextState) -> Self {
        Self {
            ctx,
            blocked,
            state,
        }
    }

    pub fn is_focused(&self) -> bool {
        self.state == UpdateContextState::Focused
    }

    pub fn is_clicked(&self, rect: Rect) -> bool {
        if !input::is_mouse_button_pressed(self.ctx, MouseButton::Left) {
            return false;
        }
        let mouse_pos = input::get_mouse_position(self.ctx);
        if !rect.contains_point(mouse_pos) {
            return false;
        }
        if self.blocked.iter().any(|r| r.contains_point(mouse_pos)) {
            return false;
        }

        true
    }

    pub fn is_hovered(&self, rect: Rect) -> bool {
        let mouse_pos = input::get_mouse_position(self.ctx);
        if !rect.contains_point(mouse_pos) {
            return false;
        }
        if self.blocked.iter().any(|r| r.contains_point(mouse_pos)) {
            return false;
        }

        true
    }
}

pub trait Update {
    // TODO: implement a way to tell there is an yes-or-no-style alert, blocking even hovering
    fn update(&mut self, _ctx: UpdateContext) -> Transition {
        Transition::None
    }
    fn block_mouse(&self) -> bool {
        true
    }
}

pub trait Disable {
    fn disabled(&self) -> bool;
    fn set_disabled(&mut self, disabled: bool);
}

pub trait Colorize {
    fn color(&self) -> Color;
    fn set_color<C: Into<Color>>(&mut self, value: C);
}

pub trait Stringify {
    fn value(&self) -> String;
    fn set_value<C: Into<String>>(&mut self, value: C);
}

pub trait Hover {
    fn on_hovered(&mut self);
    fn off_hovered(&mut self);
    fn hovered(&self) -> bool;
}

pub trait Press {
    fn on_pressed(&mut self);
    fn off_pressed(&mut self);
    fn unpress(&mut self);
    fn pressed(&self) -> bool;
}

pub trait Focus {
    fn focused(&self) -> bool {
        false
    }
    fn set_focused(&mut self, _focused: bool) {}
}

pub trait UiSprite: Draw + Positionable + Update + Focus {
    fn as_button(&mut self) -> Option<&mut Button> {
        None
    }
    fn as_text_input(&mut self) -> Option<&mut TextInput> {
        None
    }
    fn as_label(&mut self) -> Option<&mut Label> {
        None
    }
    fn as_alert(&mut self) -> Option<&mut Alert> {
        None
    }
    fn as_just_mesh(&mut self) -> Option<&mut JustMesh> {
        None
    }
    fn as_tileset_sprite(&mut self) -> Option<&mut TilesetSprite> {
        None
    }
}
