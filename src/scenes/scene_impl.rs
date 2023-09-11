use tetra::{Context, Event};

use crate::ui::{SomeUISprites, SomeUISpritesMut};

use super::{SomeTransitions, Transition};

pub trait SceneImpl {
    fn on_update(&mut self, _ctx: &mut Context) -> SomeTransitions {
        None
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> SomeTransitions {
        None
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: (i32, i32)) {}
    fn sprites(&self) -> SomeUISprites {
        None
    }
    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: u8) -> SomeTransitions {
        None
    }

    fn is_there_focused_sprite(&self) -> bool {
        self.sprites()
            .map_or(false, |sprites| sprites.iter().any(|s| s.focused()))
    }

    fn reposition_all_sprites(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        if let Some(sprites) = self.sprites_mut() {
            sprites
                .iter_mut()
                .for_each(|sprite| sprite.positionate(ctx, window_size));
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Vec<Transition> {
        // TODO: find a way to optimize this shit
        let mut transitions = self.on_update(ctx).unwrap_or_default();
        let focused = self.is_there_focused_sprite();
        if let Some(sprites) = self.sprites_mut() {
            // creating same big useless vec of Rects EVERY frame
            let mut blocked = Vec::with_capacity(sprites.len());
            sprites.iter_mut().rev().for_each(|sprite| {
                if let Some(transition) = sprite.update(ctx, focused, &blocked) {
                    transitions.push(transition);
                }
                if sprite.visible() && sprite.block_mouse() {
                    blocked.push(sprite.rect());
                }
            });
        }

        transitions
    }
}
