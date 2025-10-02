#![allow(dead_code)]

use std::rc::Rc;

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::DrawParams,
    input::{Key, MouseButton},
    Context,
};

use super::super::{
    Draw, Focus, HasLayout, HasSize, Layout, Position, Positionable, UiSprite, Update,
    UpdateContext, UpdateContextState,
};
use crate::{assets::Alert as AlertAsset, input, scenes::Transition};

pub struct Alert {
    asset: Rc<AlertAsset>,
    scale: Vec2,
    width: f32,
    height: f32,
    layout: Layout,
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
            layout: Layout::new(position),
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

    pub fn set_size(&mut self, ctx: &mut Context, window_size: Vec2) {
        self.width = window_size.x;
        self.height = window_size.y;
        self.update_position(ctx, window_size);
    }
}

impl Draw for Alert {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.layout().rect();
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

impl HasSize for Alert {
    fn size(&mut self, _ctx: &mut Context) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

impl HasLayout for Alert {
    fn layout(&self) -> &Layout {
        &self.layout
    }

    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}

impl Positionable for Alert {}

impl Update for Alert {
    fn update(&mut self, ctx: UpdateContext) -> Transition {
        if !self.active || ctx.state == UpdateContextState::Focused {
            return Transition::None;
        }
        if input::is_key_pressed(ctx.ctx, Key::Escape) || ctx.is_clicked(self.layout().rect()) {
            return Transition::Pop;
        }

        Transition::None
    }
}

impl Focus for Alert {}

impl UiSprite for Alert {
    fn as_alert(&mut self) -> Option<&mut Alert> {
        Some(self)
    }
}
