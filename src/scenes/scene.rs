use crate::app::App;
use crate::game::units::PlayerPersonality;
use crate::ui::{SomeUISprites, SomeUISpritesMut, UpdateContext, UpdateContextState};
use roguemetry::Vec2;
use std::path::PathBuf;
use tetra::{Context, Event};

use super::implements::{
    CharacterAttributes, CreateCharacter, CreateWorld, Empty, GameMenu, GameScene, LoadWorld,
    MainMenu, SettingsScene,
};
use super::Transition;

#[derive(Debug, Clone)]
pub enum SceneKind {
    MainMenu,
    #[allow(dead_code)]
    Empty,
    Settings,
    CreateWorld,
    LoadWorld,
    CreateCharacter(PathBuf),
    CharacterAttributes(PathBuf, PlayerPersonality),
    Game,
    GameMenu,
}

impl SceneKind {
    // TODO: add Result<> to all Scene::new() (why?)
    pub fn create(self, app: &App, ctx: &mut Context) -> Box<dyn Scene> {
        match self {
            SceneKind::MainMenu => Box::new(MainMenu::new(app)),
            SceneKind::Empty => Box::new(Empty::new(ctx, app)),
            SceneKind::Settings => Box::new(SettingsScene::new(app, ctx)),
            SceneKind::CreateWorld => Box::new(CreateWorld::new(app, ctx)),
            SceneKind::LoadWorld => Box::new(LoadWorld::new(app, ctx)),
            SceneKind::CreateCharacter(path) => Box::new(CreateCharacter::new(&path, app, ctx)),
            SceneKind::CharacterAttributes(path, personality) => {
                Box::new(CharacterAttributes::new(&path, personality, app, ctx))
            }
            SceneKind::Game => Box::new(GameScene::new(app)),
            SceneKind::GameMenu => Box::new(GameMenu::new(app)),
        }
    }
}

pub trait Scene {
    fn on_update(&mut self, _ctx: &mut Context) -> Transition {
        Transition::None
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> Transition {
        Transition::None
    }
    fn draw(&mut self, ctx: &mut Context);
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: Vec2) {}
    fn sprites(&self) -> SomeUISprites<'_> {
        None
    }
    fn sprites_mut(&mut self) -> SomeUISpritesMut<'_> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: u8) -> Transition {
        Transition::None
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

    fn relayout(&mut self, ctx: &mut Context, window_size: Vec2) {
        if let Some(sprites) = self.sprites_mut() {
            for sprite in sprites.iter_mut() {
                sprite.update_position(ctx, window_size);
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Transition {
        let transition = self.on_update(ctx);
        if transition.is_some() {
            return transition;
        }

        // TODO: find a way to optimize this shit
        let state = self.get_update_context_state();
        if let Some(sprites) = self.sprites_mut() {
            // creating same big useless vec of Rects EVERY frame
            let mut blocked = Vec::with_capacity(sprites.len());
            for sprite in sprites.iter_mut().rev() {
                let context = UpdateContext::new(ctx, &blocked, state);
                let transition = sprite.update(context);
                if transition.is_some() {
                    return transition;
                }
                if sprite.visible() && sprite.block_mouse() {
                    blocked.push(sprite.layout().rect());
                }
            }
        }

        Transition::None
    }
}
