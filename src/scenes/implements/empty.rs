use tetra::{Context, Event};

use super::super::{
    super::{
        app::App,
        ui::{SomeUISprites, SomeUISpritesMut, UiSprite},
    },
    helpers::{bg, easy_back},
    SceneImpl, SomeTransitions,
};

pub struct Empty {
    sprites: [Box<dyn UiSprite>; 1],
}

impl Empty {
    pub fn new(_ctx: &mut Context, app: &App) -> Self {
        Self {
            sprites: [bg(&app.assets)],
        }
    }
}

impl SceneImpl for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, false)
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }
}
