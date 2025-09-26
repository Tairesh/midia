#![allow(dead_code)]

use std::rc::Rc;

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::DrawParams,
    input::{Key, MouseButton},
    Context,
};

use crate::{assets::Alert as AlertAsset, input, scenes::Transition};

use super::super::{Draw, Focus, Position, Positionate, UiSprite, Update};

pub struct Alert {
    asset: Rc<AlertAsset>,
    scale: Vec2,
    width: f32,
    height: f32,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    active: bool,
}

impl Alert {
    pub fn new(width: f32, height: f32, asset: Rc<AlertAsset>, position: Position) -> Self {
        Alert {
            asset,
            scale: Vec2::new(3.0, 3.0),
            width,
            height,
            position,
            rect: None,
            visible: true,
            active: true,
        }
    }

    pub fn passive(width: f32, height: f32, asset: Rc<AlertAsset>, position: Position) -> Self {
        Self {
            active: false,
            ..Self::new(width, height, asset, position)
        }
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_size(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.width = window_size.0 as f32;
        self.height = window_size.1 as f32;
        self.positionate(ctx, window_size);
    }
}

impl Draw for Alert {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.asset.texture.draw_nine_slice(
            ctx,
            &self.asset.nineslice,
            self.width / self.scale.x,
            self.height / self.scale.y,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .scale(self.scale),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Alert {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        Vec2::new(self.width, self.height)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Alert {
    fn update(&mut self, ctx: &mut Context, focused: bool, blocked: &[Rect]) -> Option<Transition> {
        if !self.active {
            return None;
        }
        if focused {
            return None;
        }
        if input::is_key_pressed(ctx, Key::Escape) {
            return Some(Transition::Pop);
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let mouse = input::get_mouse_position(ctx);
            if !self.rect.unwrap().contains_point(mouse) {
                if blocked.iter().any(|r| r.contains_point(mouse)) {
                    return None;
                }
                return Some(Transition::Pop);
            }
        }
        None
    }
}

impl Focus for Alert {}

impl UiSprite for Alert {
    fn as_alert(&mut self) -> Option<&mut Alert> {
        Some(self)
    }
}
