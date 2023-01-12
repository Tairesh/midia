use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use geometry::{Direction, Point, TwoDimDirection, Vec2};
use tetra::graphics::Color;
use tetra::Context;

use crate::game::Item;
use crate::scenes::map_view;
use crate::{
    app::App,
    assets::Assets,
    colors::Colors,
    game::{map::item::ItemView, Action, ActionType, World},
    ui::{GameLog, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite, Vertical},
};

use super::super::{
    game_modes::{implements::Walking, GameMode, GameModeImpl},
    SceneImpl, SomeTransitions,
};

pub struct GameScene {
    sprites: [Box<dyn UiSprite>; 4],
    pub world: Rc<RefCell<World>>,
    pub modes: Vec<Rc<RefCell<GameMode>>>,
    pub log: GameLog,
    pub shift_of_view: Point,
    pub assets: Rc<Assets>,
    pub window_size: (i32, i32),
}

impl GameScene {
    pub fn new(app: &App) -> Self {
        let world = app.get_world();
        let name_label = Box::new(Label::new(
            world.borrow().player().person().mind.name.as_str(),
            app.assets.fonts.header.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(55.0, 8.0),
        ));
        // TODO: custom calendar
        let current_time_label = Box::new(Label::new(
            format!("{}", world.borrow().meta.current_tick),
            app.assets.fonts.default.clone(),
            Colors::WHITE_SMOKE,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 5.0 }),
        ));
        let hands_label = Box::new(Label::new(
            "Hands:",
            app.assets.fonts.default.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(5.0, 40.0),
        ));
        let hands_display = Box::new(Label::new(
            world
                .borrow()
                .player()
                .wield
                .first()
                .map_or("empty".to_string(), Item::name),
            app.assets.fonts.default.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(65.0, 40.0),
        ));
        Self {
            sprites: [name_label, hands_label, hands_display, current_time_label],
            modes: vec![Rc::new(RefCell::new(Walking::new().into()))],
            log: GameLog::new(app.assets.fonts.default.font.clone()),
            shift_of_view: Point::default(),
            assets: app.assets.clone(),
            window_size: app.window_size,
            world,
        }
    }

    pub fn current_mode(&self) -> Rc<RefCell<GameMode>> {
        self.modes.last().unwrap().clone()
    }

    pub fn push_mode(&mut self, mode: GameMode) {
        match mode.can_push(&self.world.borrow()) {
            Ok(..) => self.modes.push(Rc::new(RefCell::new(mode))),
            Err(s) => {
                self.log.log(s, Colors::LIGHT_CORAL);
            }
        }
    }

    pub fn try_rotate_player(&mut self, dir: Direction) {
        if let Ok(dir) = TwoDimDirection::try_from(dir) {
            self.world.borrow_mut().player_mut().vision = dir;
        }
    }

    pub fn examine(&mut self, dir: Direction) {
        let pos = self.world.borrow().player().pos + dir;
        self.log
            .log(self.world.borrow().this_is(pos, false), Colors::WHITE_SMOKE);
    }

    fn cancel_action_msg(&mut self, msg: String) {
        if !self.log.same_message(&msg) {
            self.log.log(msg, Colors::LIGHT_CORAL);
        }
    }

    pub fn try_start_action(&mut self, typ: ActionType) {
        let action = Action::new(0, typ, &self.world.borrow());
        match action {
            Ok(action) => {
                self.world.borrow_mut().player_mut().action = Some(action);
            }
            Err(msg) => self.cancel_action_msg(msg),
        }
    }

    pub fn mode_update(&mut self, ctx: &mut Context) -> SomeTransitions {
        self.current_mode().borrow_mut().update(ctx, self)
    }

    pub fn tile_size(&self) -> f32 {
        self.assets.tileset.tile_size as f32 * self.world.borrow().game_view.zoom.as_view()
    }

    fn make_world_tick(&mut self, ctx: &mut Context) {
        self.world.borrow_mut().tick();

        for event in self.world.borrow().log().new_events() {
            self.log.log(event.msg.as_str(), event.category.into());
        }
        let current_time = format!("{}", self.world.borrow().meta.current_tick);
        let hands_display = self
            .world
            .borrow()
            .player()
            .wield
            .first()
            .map_or("empty".to_string(), Item::name);
        let window_size = self.window_size;
        self.current_time_label()
            .update(current_time, ctx, window_size);
        self.hands_display_label()
            .update(hands_display, ctx, window_size);
    }

    fn hands_display_label(&mut self) -> &mut Label {
        self.sprites[2].as_label().unwrap()
    }

    fn current_time_label(&mut self) -> &mut Label {
        self.sprites[3].as_label().unwrap()
    }

    fn cursors(&self) -> Vec<(Point, Color)> {
        self.current_mode().borrow().cursors(&self.world.borrow())
    }
}

impl SceneImpl for GameScene {
    fn on_update(&mut self, ctx: &mut Context) -> SomeTransitions {
        if self.world.borrow().player().action.is_some() {
            self.make_world_tick(ctx);

            None
        } else {
            self.mode_update(ctx)
        }
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        map_view::view::draw(
            ctx,
            &self.world,
            &self.assets,
            self.window_size,
            self.shift_of_view,
            self.cursors(),
        );
    }

    fn after_draw(&mut self, ctx: &mut Context) {
        // TODO: move this to UI

        map_view::ui::draw_log(ctx, &mut self.log);

        map_view::view::draw_unit(
            ctx,
            &self.assets.tileset,
            Vec2::new(5.0, 5.0),
            3.0,
            false,
            self.world.borrow().player(),
        );
        self.current_mode().borrow_mut().draw(ctx, self);
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }
}

impl Drop for GameScene {
    fn drop(&mut self) {
        self.world.borrow_mut().save();
    }
}
