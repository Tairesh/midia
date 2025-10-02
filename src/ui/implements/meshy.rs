#![allow(dead_code)]

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::{mesh::Mesh, Color, DrawParams},
    input, Context,
};

use super::super::{
    Colorize, Draw, Focus, Hover, Position, Positionable, Sizeable, UiSprite, Update,
};
use crate::scenes::Transition;
use crate::ui::UpdateContext;

pub struct JustMesh {
    mesh: Mesh,
    color: Option<Color>,
    size: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    scale: Vec2,
}

impl JustMesh {
    pub fn new(mesh: Mesh, color: Option<Color>, size: Vec2, position: Position) -> Self {
        Self {
            mesh,
            color,
            size,
            position,
            rect: None,
            visible: true,
            scale: Vec2::one(),
        }
    }
}

impl Draw for JustMesh {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.mesh.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .scale(self.scale)
                .color(self.color.unwrap_or(Color::WHITE)),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Sizeable for JustMesh {
    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        self.size * self.scale
    }
}

impl Positionable for JustMesh {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for JustMesh {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color<C: Into<Color>>(&mut self, value: C) {
        self.color = Some(value.into());
    }
}

impl Update for JustMesh {}

impl Focus for JustMesh {}

impl UiSprite for JustMesh {
    fn as_just_mesh(&mut self) -> Option<&mut JustMesh> {
        Some(self)
    }
}

pub struct HoverableMesh {
    mesh: Mesh,
    bg_color: Color,
    bg_color_hover: Color,
    size: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    is_hovered: bool,
}

impl HoverableMesh {
    pub fn new(
        mesh: Mesh,
        bg_color: Color,
        bg_color_hover: Color,
        size: Vec2,
        position: Position,
    ) -> Self {
        Self {
            mesh,
            bg_color,
            bg_color_hover,
            size,
            position,
            rect: None,
            visible: true,
            is_hovered: false,
        }
    }
}

impl Draw for HoverableMesh {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.mesh.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .color(if self.is_hovered {
                    self.bg_color_hover
                } else {
                    self.bg_color
                }),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Sizeable for HoverableMesh {
    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        self.size
    }
}

impl Positionable for HoverableMesh {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for HoverableMesh {
    fn update(&mut self, ctx: UpdateContext) -> Transition {
        let mouse = input::get_mouse_position(ctx.ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        // no check for blocking cause it uses as background light
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        Transition::None
    }
}

impl Hover for HoverableMesh {
    fn on_hovered(&mut self) {
        self.is_hovered = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
    }

    fn hovered(&self) -> bool {
        self.is_hovered
    }
}

impl Focus for HoverableMesh {}

impl UiSprite for HoverableMesh {}
