use std::collections::VecDeque;

use geometry::Vec2;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::Context;

#[derive(Debug)]
pub struct LogMessageText {
    pub text: Text,
    pub color: Color,
}

impl LogMessageText {
    pub fn new<S: Into<String>>(content: S, font: Font, color: Color) -> Self {
        Self {
            text: Text::new(content, font),
            color,
        }
    }

    pub fn draw(&mut self, position: Vec2, ctx: &mut Context) {
        self.text
            .draw(ctx, DrawParams::new().position(position).color(self.color));
    }
}

#[derive(Debug)]
pub struct GameLog {
    pub texts: VecDeque<LogMessageText>,
    font: Font,
}

impl GameLog {
    const TEXTS_LIMIT: usize = 5;

    pub fn new(font: Font) -> Self {
        Self {
            texts: VecDeque::with_capacity(Self::TEXTS_LIMIT),
            font,
        }
    }

    pub fn log<S: Into<String>>(&mut self, message: S, color: Color) {
        if self.texts.len() >= Self::TEXTS_LIMIT {
            self.texts.pop_back();
        }
        self.texts
            .push_front(LogMessageText::new(message, self.font.clone(), color));
    }

    pub fn clear(&mut self) {
        self.texts.clear();
    }

    pub fn same_message(&self, new_msg: &String) -> bool {
        self.texts
            .front()
            .map_or(false, |t| new_msg == t.text.content())
    }
}
