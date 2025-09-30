use tetra::{Context, Event};

use crate::ui::{SomeUISprites, SomeUISpritesMut, UpdateContext, UpdateContextState};

use super::Transition;

pub trait SceneImpl {
    fn on_update(&mut self, _ctx: &mut Context) -> Option<Transition> {
        None
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> Option<Transition> {
        None
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: (i32, i32)) {}
    fn sprites(&self) -> SomeUISprites<'_> {
        None
    }
    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: u8) -> Option<Transition> {
        None
    }

    fn get_update_context_state(&self) -> UpdateContextState {
        if self
            .sprites()
            .is_some_and(|sprites| sprites.iter().any(|s| s.focused()))
        {
            UpdateContextState::Focused
        } else {
            UpdateContextState::Normal
        }
    }

    fn reposition_all_sprites(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        if let Some(sprites) = self.sprites_mut() {
            for sprite in sprites.iter_mut() {
                sprite.positionate(ctx, window_size);
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if let Some(transition) = self.on_update(ctx) {
            return Some(transition);
        }

        // TODO: find a way to optimize this shit
        let state = self.get_update_context_state();
        if let Some(sprites) = self.sprites_mut() {
            // creating same big useless vec of Rects EVERY frame
            let mut blocked = Vec::with_capacity(sprites.len());
            for sprite in sprites.iter_mut().rev() {
                let context = UpdateContext::new(ctx, &blocked, state);
                if let Some(transition) = sprite.update(context) {
                    return Some(transition);
                }
                if sprite.visible() && sprite.block_mouse() {
                    blocked.push(sprite.rect());
                }
            }
        }

        None
    }
}
