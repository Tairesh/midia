use tetra::{Context, Event};

use super::super::{super::app::App, helpers::easy_back, Scene};
use crate::scenes::Transition;
use crate::ui::{Draw, Image};

pub struct Empty {
    bg: Image,
}

impl Empty {
    pub fn new(_ctx: &mut Context, app: &App) -> Self {
        Self {
            bg: Image::auto_size(app.assets.images.bg.clone()),
        }
    }
}

impl Scene for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> Transition {
        easy_back(&event)
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.bg.draw(ctx);
    }
}
