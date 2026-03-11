use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use roguemetry::Vec2;
use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        Rectangle,
    },
    input::{Key, KeyModifier},
    Context, Event,
};

use crate::{
    app::App,
    colors::Colors,
    savefile::{self, savefiles, savefiles_exists, Meta},
    ui::{
        draw_sprites, Alert, ButtonBuilder, HasSize, Horizontal, HoverableMesh, Label, Position,
        UISpritesCollection, UiSprite, Vertical,
    },
    VERSION,
};

use super::super::{helpers::easy_back, Scene, SceneKind, Transition};

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

const ROW_HEIGHT: f32 = 50.0;
const ALERT_WIDTH: f32 = 600.0;
const ROW_WIDTH: f32 = 564.0;

type Sprites = Vec<Box<dyn UiSprite>>;

pub struct LoadWorld {
    sprites: Sprites,
    paths: Vec<PathBuf>,
}

impl LoadWorld {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let savefiles = savefiles();
        let height = savefiles.len() as f32 * ROW_HEIGHT + 33.0;
        let start_y = -height / 2.0;

        let mut sprites: Sprites = Vec::with_capacity(savefiles.len() * 6 + 1);
        sprites.push(Box::new(Alert::new(
            ALERT_WIDTH,
            height,
            app.assets.alert.clone(),
            Position::new(
                Horizontal::CenterByCenter,
                Vertical::CenterByTop,
                Vec2::new(0.0, start_y - 18.0),
            ),
        )));

        for (i, savefile) in savefiles.iter().enumerate() {
            let y = start_y + i as f32 * ROW_HEIGHT;
            Self::add_row_sprites(&mut sprites, i, savefile, y, ctx, app);
        }

        Self {
            sprites,
            paths: savefiles.into_iter().map(|s| s.path).collect(),
        }
    }

    fn add_row_sprites(
        sprites: &mut Sprites,
        i: usize,
        savefile: &Meta,
        y: f32,
        ctx: &mut Context,
        app: &App,
    ) {
        let bg_color = if i % 2 == 1 {
            Colors::DARK_GRAY.with_alpha(0.3)
        } else {
            Colors::TRANSPARENT
        };
        let version_color = if savefile.version.as_str() == VERSION {
            Colors::GREEN
        } else {
            Colors::RED
        };
        let key_label = |n: usize| if n < 9 { n + 1 } else { 0 };

        // Background row
        sprites.push(Box::new(HoverableMesh::new(
            Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, ROW_WIDTH, ROW_HEIGHT),
            )
            .unwrap(),
            bg_color,
            Colors::KHAKI.with_alpha(0.6),
            Vec2::new(560.0, ROW_HEIGHT),
            Position::new(
                Horizontal::CenterByLeft,
                Vertical::CenterByTop,
                Vec2::new(-282.0, y),
            ),
        )));

        // Savefile name
        sprites.push(Box::new(Label::new(
            savefile.name.as_str(),
            app.assets.fonts.header.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(
                Horizontal::CenterByLeft,
                Vertical::CenterByTop,
                Vec2::new(-280.0, y - 2.0),
            ),
        )));

        // Version label
        let mut version_label = Box::new(Label::new(
            savefile.version.as_str(),
            app.assets.fonts.default.clone(),
            version_color,
            Position::new(
                Horizontal::CenterByLeft,
                Vertical::CenterByTop,
                Vec2::new(-275.0, y + 30.0),
            ),
        ));
        let version_width = version_label.size(ctx).x;
        sprites.push(version_label);

        // Date/time label
        let time: DateTime<Local> = savefile.time.into();
        sprites.push(Box::new(Label::new(
            time.format("%Y.%m.%d %H:%M:%S").to_string(),
            app.assets.fonts.default.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(
                Horizontal::CenterByLeft,
                Vertical::CenterByTop,
                Vec2::new(-270.0 + version_width, y + 30.0),
            ),
        )));

        // Load button
        let (load_text, load_keys) = if i < 10 {
            (format!("[{}] Load", key_label(i)), vec![KEYS[i].into()])
        } else {
            ("Load".to_string(), vec![])
        };
        sprites.push(Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text(load_text, app.assets.fonts.default.clone())
                .with_position(Position::new(
                    Horizontal::CenterByRight,
                    Vertical::CenterByCenter,
                    Vec2::new(120.0, y + 24.5),
                ))
                .with_keys(load_keys)
                .with_transition(Transition::CustomEvent((i * 2) as u8))
                .build(),
        ));

        // Delete button
        let (delete_text, delete_keys) = if i < 10 {
            (
                format!("[Alt+{}] Delete", key_label(i)),
                vec![(KEYS[i], KeyModifier::Alt).into()],
            )
        } else {
            ("Delete".to_string(), vec![])
        };
        sprites.push(Box::new(
            ButtonBuilder::new(app.assets.button.clone())
                .with_text(delete_text, app.assets.fonts.default.clone())
                .with_position(Position::new(
                    Horizontal::CenterByRight,
                    Vertical::CenterByCenter,
                    Vec2::new(275.0, y + 24.5),
                ))
                .with_keys(delete_keys)
                .with_transition(Transition::CustomEvent((i * 2 + 1) as u8))
                .build(),
        ));
    }
}

impl Scene for LoadWorld {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> Transition {
        easy_back(&event)
    }

    fn draw(&mut self, ctx: &mut Context) {
        draw_sprites(ctx, &mut self.sprites);
    }

    fn sprites_mut(&mut self) -> UISpritesCollection<'_> {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> Transition {
        let i = (event / 2) as usize;
        let path = self.paths.get(i).expect("Invalid savefile index");

        if event.is_multiple_of(2) {
            load_savefile(path)
        } else {
            delete_savefile(path)
        }
    }
}

fn load_savefile(path: &Path) -> Transition {
    if savefile::load(path).is_some() {
        if savefile::has_avatar(path) {
            Transition::Push(SceneKind::Game(path.to_path_buf()))
        } else {
            Transition::Switch(SceneKind::CreateCharacter(path.to_path_buf()))
        }
    } else {
        panic!("Can't load savefile: {}", path.display());
    }
}

fn delete_savefile(path: &Path) -> Transition {
    savefile::delete(path);
    if savefiles_exists() {
        Transition::Switch(SceneKind::LoadWorld)
    } else {
        Transition::Pop
    }
}
