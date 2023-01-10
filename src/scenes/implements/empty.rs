use tetra::{graphics, Context, Event};

use crate::colors::Colors;

use super::super::{helpers::easy_back, SceneImpl, SomeTransitions};

pub struct Empty {}

impl SceneImpl for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, false)
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::DARK_BROWN);
    }
}
