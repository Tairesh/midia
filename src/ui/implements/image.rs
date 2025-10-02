#![allow(dead_code)]

use std::rc::Rc;

use roguemetry::{Rect, Vec2};
use tetra::{
    graphics::{Color, DrawParams, NineSlice, Rectangle, Texture},
    window, Context,
};

use crate::assets::{Sprite, Tileset};

use super::super::{Colorize, Draw, Focus, Position, Positionable, Sizeable, UiSprite, Update};

pub struct Image {
    texture: Texture,
    region: Option<Rectangle>,
    color: Option<Color>,
    nine_slice: Option<(NineSlice, f32, f32)>,
    scale: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    repeat: bool,
    auto_size: bool,
    window_size: (i32, i32),
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Self {
        Image {
            texture,
            region: None,
            color: None,
            nine_slice: None,
            scale: Vec2::new(1.0, 1.0),
            position,
            rect: None,
            visible: true,
            repeat: false,
            auto_size: false,
            window_size: (0, 0),
        }
    }

    pub fn repeat(texture: Texture) -> Self {
        Self {
            repeat: true,
            ..Self::new(texture, Position::center())
        }
    }

    pub fn auto_size(texture: Texture) -> Self {
        Self {
            auto_size: true,
            ..Self::new(texture, Position::center())
        }
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_nineslice(mut self, nineslice: NineSlice, width: f32, height: f32) -> Self {
        self.nine_slice = Some((nineslice, width, height));
        self
    }
}

impl Draw for Image {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let params = DrawParams::new()
            .position(Vec2::new(rect.x, rect.y))
            .scale(self.scale)
            .color(self.color.unwrap_or(Color::WHITE));
        if let Some((nine_slice, width, height)) = &self.nine_slice {
            self.texture
                .draw_nine_slice(ctx, nine_slice, *width, *height, params);
        } else if let Some(region) = self.region {
            self.texture.draw_region(ctx, region, params);
        } else if self.repeat {
            let (w, h) = self.window_size;
            let w_count = ((w as f32 / rect.w).ceil() / 2.0) as i32;
            let h_count = ((h as f32 / rect.h).ceil() / 2.0) as i32;
            for i in -w_count..=w_count {
                for j in -h_count..=h_count {
                    let pos = Vec2::new(rect.x + i as f32 * rect.w, rect.y + j as f32 * rect.h);
                    self.texture.draw(ctx, params.clone().position(pos));
                }
            }
        } else {
            self.texture.draw(ctx, params);
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Sizeable for Image {
    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        if self.repeat || self.auto_size {
            self.window_size = window::get_size(ctx);
        }
        let size = if let Some(region) = self.region {
            (region.width, region.height)
        } else if self.auto_size {
            let (w, h) = self.texture.size();
            let kx = self.window_size.0 as f32 / w as f32;
            let ky = self.window_size.1 as f32 / h as f32;
            let k = kx.max(ky) + 0.1;
            self.scale = Vec2::new(k, k);
            (w as f32, h as f32)
        } else {
            let (w, h) = self.texture.size();
            (w as f32, h as f32)
        };
        Vec2::new(size.0 * self.scale.x, size.1 * self.scale.y)
    }
}

impl Positionable for Image {
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

impl Colorize for Image {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color<C: Into<Color>>(&mut self, value: C) {
        self.color = Some(value.into());
    }
}

impl Update for Image {}

impl Focus for Image {}

impl UiSprite for Image {}

pub struct TilesetSprite {
    sprite: Sprite,
    scale: Vec2,
    tileset: Rc<Tileset>,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    color: Option<Color>,
}

impl TilesetSprite {
    pub fn new(
        sprite: impl Into<Sprite>,
        tileset: Rc<Tileset>,
        position: Position,
        scale: f32,
        color: Option<Color>,
    ) -> Self {
        TilesetSprite {
            sprite: sprite.into(),
            scale: Vec2::new(scale, scale),
            tileset,
            position,
            rect: None,
            visible: true,
            color,
        }
    }

    pub fn set_sprite(&mut self, sprite: impl Into<Sprite>) {
        self.sprite = sprite.into();
    }

    pub fn remove_color(&mut self) {
        self.color = None;
    }
}

impl Draw for TilesetSprite {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut params = DrawParams::new()
            .position(Vec2::new(rect.x, rect.y))
            .scale(self.scale);
        if let Some(color) = self.color {
            params = params.color(color);
        }
        self.tileset.draw_sprite(ctx, self.sprite, params);
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Sizeable for TilesetSprite {
    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        let size = Tileset::get_size(self.sprite);
        size * self.scale
    }
}

impl Positionable for TilesetSprite {
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

impl Colorize for TilesetSprite {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color<C: Into<Color>>(&mut self, value: C) {
        self.color = Some(value.into());
    }
}

impl Update for TilesetSprite {}

impl Focus for TilesetSprite {}

impl UiSprite for TilesetSprite {
    fn as_tileset_sprite(&mut self) -> Option<&mut TilesetSprite> {
        Some(self)
    }
}
