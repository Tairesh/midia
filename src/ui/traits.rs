use geometry::{Rect, Vec2};
use tetra::graphics::Color;
use tetra::Context;

use crate::scenes::Transition;
use crate::ui::JustMesh;

use super::{Alert, Button, Label, Position, TextInput};

pub trait Draw {
    fn draw(&mut self, ctx: &mut Context);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub trait Positionate {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn calc_size(&mut self, ctx: &mut Context) -> Vec2;
    fn rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
    fn calc_rect(&mut self, owner_size: Vec2, window_size: (i32, i32)) -> Rect {
        let left_top = self.position().as_vec(owner_size, window_size);
        Rect::new(left_top.x, left_top.y, owner_size.x, owner_size.y)
    }
    fn positionate(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        let size = self.calc_size(ctx);
        let rect = self.calc_rect(size, window_size);
        self.set_rect(rect);
    }
}

pub trait Update {
    // focused means there is some focused sprite on the scene, that intercepts all input
    // blocked is rects of ui elements above current one
    // TODO: implement a way to tell there is an yes-or-no-style alert, blocking even hovering
    fn update(
        &mut self,
        _ctx: &mut Context,
        _focused: bool,
        _blocked: &[Rect],
    ) -> Option<Transition> {
        None
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

pub trait UiSprite: Draw + Positionate + Update + Focus {
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
}
