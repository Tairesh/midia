use std::path::PathBuf;

use tetra::Context;

use crate::app::App;
use crate::game::races::Personality;

use super::{
    implements::{
        CharacterAttributes, CreateCharacter, CreateWorld, Empty, GameMenu, GameScene, LoadWorld,
        MainMenu, SettingsScene,
    },
    SceneImpl,
};

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    #[allow(dead_code)]
    Empty,
    Settings,
    CreateWorld,
    LoadWorld,
    CreateCharacter(PathBuf),
    CharacterAttributes(PathBuf, Personality),
    GameScene,
    GameMenu,
}

impl Scene {
    // TODO: add Result<> to all Scene::new() (why?)
    pub fn into_impl(self, app: &App, ctx: &mut Context) -> Box<dyn SceneImpl> {
        match self {
            Scene::MainMenu => Box::new(MainMenu::new(app)),
            Scene::Empty => Box::new(Empty::new(ctx, app)),
            Scene::Settings => Box::new(SettingsScene::new(app, ctx)),
            Scene::CreateWorld => Box::new(CreateWorld::new(app, ctx)),
            Scene::LoadWorld => Box::new(LoadWorld::new(app, ctx)),
            Scene::CreateCharacter(path) => Box::new(CreateCharacter::new(&path, app, ctx)),
            Scene::CharacterAttributes(path, personality) => {
                Box::new(CharacterAttributes::new(&path, personality, app, ctx))
            }
            Scene::GameScene => Box::new(GameScene::new(app)),
            Scene::GameMenu => Box::new(GameMenu::new(app)),
        }
    }
}
