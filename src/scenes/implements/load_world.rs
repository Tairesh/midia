use std::path::PathBuf;

use geometry::Vec2;
use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        Rectangle,
    },
    input::{Key, KeyModifier},
    Context, Event,
};
use time::{format_description::FormatItem, OffsetDateTime};

use crate::{
    app::App,
    colors::Colors,
    savefile::{self, savefiles, savefiles_exists, Meta},
    settings::Settings,
    ui::{
        Alert, Button, Horizontal, HoverableMesh, Label, Position, Positionate, SomeUISprites,
        SomeUISpritesMut, UiSprite, Vertical,
    },
    VERSION,
};

use super::super::{helpers::easy_back, Scene, SceneImpl, SomeTransitions, Transition};

const KEYS: [Key; 10] = [
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
    Key::Num0,
];

const DATETIME_FORMAT: &[FormatItem] =
    time::macros::format_description!("[year].[month].[day] [hour]:[minute]:[second]");

type Sprites = Vec<Box<dyn UiSprite>>;

pub struct LoadWorld {
    sprites: Sprites,
    paths: Vec<PathBuf>,
}

impl LoadWorld {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let savefiles = savefiles();
        let mut sprites: Sprites = Vec::with_capacity(savefiles.len() * 6 + 1);
        let height = savefiles.len() as f32 * 50.0 + 33.0;
        // TODO: Add scroll if there are too many savefiles
        let mut y = -height / 2.0;

        sprites.push(Box::new(Alert::new(
            600.0,
            height,
            app.assets.alert.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByTop { offset: y - 18.0 },
            },
        )));
        for (i, savefile) in savefiles.iter().enumerate() {
            Self::push_sprites_for_savefile(&mut sprites, i, savefile, y, ctx, app);
            y += 50.0;
        }

        Self {
            sprites,
            paths: savefiles.into_iter().map(|s| s.path).collect(),
        }
    }

    fn push_sprites_for_savefile(
        sprites: &mut Sprites,
        i: usize,
        savefile: &Meta,
        y: f32,
        ctx: &mut Context,
        app: &App,
    ) {
        sprites.push(Box::new(HoverableMesh::new(
            Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 564.0, 50.0)).unwrap(),
            if i % 2 == 1 {
                Colors::DARK_GRAY.with_alpha(0.3)
            } else {
                Colors::TRANSPARENT
            },
            Colors::KHAKI.with_alpha(0.6),
            Vec2::new(560.0, 50.0),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -282.0 },
                y: Vertical::AtWindowCenterByTop { offset: y },
            },
        )));
        sprites.push(Box::new(Label::new(
            savefile.name.as_str(),
            app.assets.fonts.header.clone(),
            Colors::LIGHT_YELLOW,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -280.0 },
                y: Vertical::AtWindowCenterByTop { offset: y - 2.0 },
            },
        )));
        let mut version_label = Box::new(Label::new(
            savefile.version.as_str(),
            app.assets.fonts.default.clone(),
            if savefile.version.as_str() == VERSION {
                Colors::GREEN
            } else {
                Colors::RED
            },
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -275.0 },
                y: Vertical::AtWindowCenterByTop { offset: y + 30.0 },
            },
        ));
        let version_label_size = version_label.calc_size(ctx);
        sprites.push(version_label);
        let time = OffsetDateTime::from(savefile.time).to_offset(Settings::instance().time.offset);
        sprites.push(Box::new(Label::new(
            time.format(&DATETIME_FORMAT).unwrap(),
            app.assets.fonts.default.clone(),
            Colors::LIGHT_YELLOW,
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: -270.0 + version_label_size.x,
                },
                y: Vertical::AtWindowCenterByTop { offset: y + 30.0 },
            },
        )));
        sprites.push(Box::new(Button::text(
            if i < 10 { vec![KEYS[i].into()] } else { vec![] },
            if i < 10 {
                format!("[{}] Load", if i < 9 { i + 1 } else { 0 })
            } else {
                "Load".to_string()
            },
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 120.0 },
                y: Vertical::AtWindowCenterByCenter { offset: y + 24.5 },
            },
            Transition::CustomEvent((i * 2) as u8),
        )));
        sprites.push(Box::new(Button::text(
            if i < 10 {
                vec![(KEYS[i], KeyModifier::Alt).into()]
            } else {
                vec![]
            },
            if i < 10 {
                format!("[Alt+{}] Delete", if i < 9 { i + 1 } else { 0 })
            } else {
                "Delete".to_string()
            },
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 275.0 },
                y: Vertical::AtWindowCenterByCenter { offset: y + 24.5 },
            },
            Transition::CustomEvent((i * 2 + 1) as u8),
        )));
    }
}

impl SceneImpl for LoadWorld {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, false)
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> SomeTransitions {
        let i = (event / 2) as usize;
        let path = self.paths.get(i)?;
        if event % 2 == 0 {
            // load
            if let Some(meta) = savefile::load(path) {
                if savefile::has_avatar(path) {
                    Some(vec![
                        Transition::LoadWorld(path.clone()),
                        Transition::Replace(Scene::GameScene),
                    ])
                } else {
                    Some(vec![Transition::Replace(Scene::CreateCharacter(meta.path))])
                }
            } else {
                panic!("Can't load savefile: {path:?}")
            }
        } else {
            // delete
            savefile::delete(path);
            if savefiles_exists() {
                Some(vec![Transition::Replace(Scene::LoadWorld)])
            } else {
                Some(vec![Transition::Pop])
            }
        }
    }
}
