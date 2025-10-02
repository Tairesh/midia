use roguemetry::Vec2;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tetra::{input::Key, window, Context, Event, State};

use crate::{
    assets::Assets,
    colors::Colors,
    game::world::World,
    savefile,
    scenes::{Scene, SceneKind, Transition},
    settings::Settings,
    ui::{Draw, Label, Position, Positionable, Stringify},
};

pub struct App {
    pub assets: Rc<Assets>,
    pub window_size: (i32, i32),
    pub world: Option<Rc<RefCell<World>>>,
    scenes: Vec<Box<dyn Scene>>,
    fps_counter: Label,
}

impl App {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let assets = Assets::load(ctx)?;
        let fps_counter = Label::new(
            "00",
            assets.fonts.default.clone(),
            Colors::LIME,
            Position::by_right_top(Vec2::new(-10.0, 10.0)),
        );
        let mut app = Self {
            assets: Rc::new(assets),
            scenes: vec![],
            window_size: window::get_size(ctx),
            world: None,
            fps_counter,
        };
        app.push_scene(ctx, SceneKind::MainMenu);
        Ok(app)
    }

    fn current_scene(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scenes.last_mut()
    }

    fn on_open(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.current_scene() {
            scene.on_open(ctx);
        }
        self.on_resize(ctx, self.window_size);
    }

    fn on_resize(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
        if let Some(scene) = self.current_scene() {
            scene.relayout(ctx, window_size);
            scene.on_resize(ctx, window_size);
            self.fps_counter.update_position(ctx, window_size);
        }
    }

    fn pop_scene(&mut self, ctx: &mut Context) {
        self.scenes.pop();
        self.on_open(ctx);
    }

    fn switch_scene(&mut self, ctx: &mut Context, scene: SceneKind) {
        self.scenes.pop();
        self.push_scene(ctx, scene);
    }

    fn push_scene(&mut self, ctx: &mut Context, scene: SceneKind) {
        self.scenes.push(scene.create(self, ctx));
        self.on_open(ctx);
    }

    fn transit(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::None => {}
            Transition::Push(s) => self.push_scene(ctx, s),
            Transition::Pop => self.pop_scene(ctx),
            Transition::Switch(s) => self.switch_scene(ctx, s),
            Transition::CustomEvent(event) => {
                if let Some(scene) = self.current_scene() {
                    let transition = scene.custom_event(ctx, event);
                    self.transit(ctx, transition);
                }
            }
            Transition::Quit => window::quit(ctx),
            Transition::ExitToMainMenu => {
                self.unload_world();
                self.scenes.drain(1..);
                self.on_open(ctx);
            }
            Transition::LoadWorld(path) => self.load_world(ctx, &path),
        }
    }

    fn load_world(&mut self, ctx: &mut Context, path: &Path) {
        match savefile::load_world(path) {
            Ok(world) => {
                self.world = Some(Rc::new(RefCell::new(world)));
                self.push_scene(ctx, SceneKind::Game);
            }
            Err(e) => {
                println!("Failed to load world: {e:?}");
            }
        }
    }

    fn unload_world(&mut self) {
        self.world = None;
    }

    pub fn get_world(&self) -> Rc<RefCell<World>> {
        if let Some(world) = &self.world {
            world.clone()
        } else {
            panic!("World isn't loaded!")
        }
    }
}

impl State for App {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(scene) = self.current_scene() {
            let transition = scene.update(ctx);
            self.transit(ctx, transition);
        } else {
            self.transit(ctx, Transition::Quit);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(scene) = self.current_scene() {
            scene.before_draw(ctx);
            if let Some(sprites) = scene.sprites_mut() {
                for sprite in &mut *sprites {
                    if sprite.visible() {
                        sprite.draw(ctx);
                    }
                }
            }
            scene.after_draw(ctx);
        }
        if Settings::instance().debug.show_fps {
            let fps = (tetra::time::get_fps(ctx).round() as u8).to_string();
            if !self.fps_counter.value().eq(&fps) {
                self.fps_counter.set_value(fps);
            }
            self.fps_counter.draw(ctx);
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                Settings::instance().debug.show_fps ^= true; // ^_^
            }
            Event::Resized { width, height } => {
                if !window::is_fullscreen(ctx) {
                    let mut settings = Settings::instance();
                    settings.window.width = width;
                    settings.window.height = height;
                }
                self.on_resize(ctx, (width, height));
            }
            _ => {}
        }

        if let Some(scene) = self.current_scene() {
            let transition = scene.event(ctx, event);
            self.transit(ctx, transition);
        }

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        Settings::instance().save();
    }
}
