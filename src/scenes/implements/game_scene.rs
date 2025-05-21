use std::cell::RefCell;
use std::rc::Rc;

use geometry::{Direction, Point, Vec2};
use tetra::{
    graphics::Canvas,
    input::{Key, KeyModifier},
    Context,
};

use crate::{
    app::App,
    assets::Assets,
    colors::Colors,
    game::{
        log::LogCategory,
        traits::{LooksLike, Name},
        Action, ActionType, Avatar, Item, World,
    },
    input,
    settings::Settings,
    ui::{
        Colorize, GameLog, Horizontal, Label, Position, SomeUISprites, SomeUISpritesMut,
        TilesetSprite, UiSprite, Vertical,
    },
};

use super::super::{
    game_modes::{implements::Walking, Cursor, GameMode, GameModeImpl},
    map_view, Scene, SceneImpl, Transition,
};

pub struct GameScene {
    sprites: [Box<dyn UiSprite>; 5],
    pub world: Rc<RefCell<World>>,
    pub modes: Vec<Rc<RefCell<GameMode>>>,
    pub log: GameLog,
    shift_of_view: Point,
    pub assets: Rc<Assets>,
    pub window_size: (i32, i32),
    need_redraw: bool,
    map_canvas: Option<Canvas>,
}

impl GameScene {
    // TODO: refactor this method
    #[allow(clippy::too_many_lines)]
    pub fn new(app: &App) -> Self {
        let world = app.get_world();
        let world_borrow = world.borrow();
        let units = world_borrow.units();
        let player = units.player();
        let name_label = Box::new(Label::new(
            player.personality.mind.name.as_str(),
            app.assets.fonts.header.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(55.0, 8.0),
        ));
        // TODO: custom calendar
        let current_time_label = Box::new(Label::new(
            format!("{}", world_borrow.meta.current_tick),
            app.assets.fonts.default.clone(),
            Colors::WHITE_SMOKE,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 5.0 }),
        ));
        let wield_label = Box::new(Label::new(
            "Wield:",
            app.assets.fonts.default.clone(),
            Colors::LIME,
            Position {
                x: Horizontal::ByLeft { x: 5.0 },
                y: Vertical::ByCenter { y: 50.0 },
            },
        ));
        let main_hand = player.inventory.main_hand();
        let main_hand_image = Box::new(TilesetSprite::new(
            main_hand.map(Item::looks_like).unwrap_or_default(),
            app.assets.tileset.clone(),
            Position {
                x: Horizontal::ByLeft { x: 60.0 },
                y: Vertical::ByCenter { y: 47.0 },
            },
            2.0,
            main_hand.map(Item::color),
        ));
        let main_hand_display = Box::new(Label::new(
            main_hand.map_or("nothing", Item::name),
            app.assets.fonts.default.clone(),
            Colors::LIME,
            Position {
                x: Horizontal::ByLeft { x: 83.0 },
                y: Vertical::ByCenter { y: 50.0 },
            },
        ));

        drop(units);
        drop(world_borrow);
        Self {
            sprites: [
                name_label,
                wield_label,
                main_hand_image,
                main_hand_display,
                current_time_label,
            ],
            modes: vec![Rc::new(RefCell::new(Walking::new().into()))],
            log: GameLog::new(app.assets.fonts.default.font.clone()),
            shift_of_view: Point::default(),
            assets: app.assets.clone(),
            window_size: app.window_size,
            world,
            need_redraw: true,
            map_canvas: None,
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
        if self
            .world
            .borrow_mut()
            .units_mut()
            .player_mut()
            .view
            .try_set_direction(dir)
        {
            self.need_redraw = true;
        }
    }

    pub fn examine(&mut self, dir: Direction) {
        let pos = self.world.borrow().units().player().pos + dir;
        self.log
            .log(self.world.borrow().this_is(pos, false), Colors::WHITE_SMOKE);
        self.need_redraw = true;
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
                self.world
                    .borrow_mut()
                    .units_mut()
                    .player_mut()
                    .set_action(Some(action));
                self.need_redraw = true;
            }
            Err(msg) => self.cancel_action_msg(msg),
        }
    }

    pub fn set_shift_of_view(&mut self, shift: Point) {
        self.shift_of_view = shift;
        self.need_redraw = true;
    }

    pub fn shift_of_view(&self) -> Point {
        self.shift_of_view
    }

    pub fn mode_update(&mut self, ctx: &mut Context) -> Option<Transition> {
        self.current_mode().borrow_mut().update(ctx, self)
    }

    pub fn tile_size(&self) -> f32 {
        self.assets.tileset.tile_size as f32 * self.world.borrow().game_view.zoom.as_view()
    }

    fn make_world_tick(&mut self, ctx: &mut Context) {
        self.world.borrow_mut().tick();

        self.update_ui(ctx);
    }

    fn main_hand_display_label(&mut self) -> &mut Label {
        self.sprites[3].as_label().unwrap()
    }

    fn main_hand_display_image(&mut self) -> &mut TilesetSprite {
        self.sprites[2].as_tileset_sprite().unwrap()
    }

    fn current_time_label(&mut self) -> &mut Label {
        self.sprites[4].as_label().unwrap()
    }

    fn cursors(&self) -> Option<Vec<Cursor>> {
        self.current_mode().borrow().cursors(&self.world.borrow())
    }

    pub fn update_ui(&mut self, ctx: &mut Context) {
        // TODO: refactor
        self.need_redraw = true;

        for event in self.world.borrow().log().new_events() {
            if event.category == LogCategory::Debug && !Settings::instance().debug.show_debug_log {
                continue;
            }
            self.log.log(event.msg.as_str(), event.category.into());
        }

        let window_size = self.window_size;
        let current_time = format!("{}", self.world.borrow().meta.current_tick);
        self.current_time_label()
            .update(current_time, ctx, window_size);

        let world = self.world.borrow();
        let units = world.units();
        let main_hand_item = units.player().inventory.main_hand();
        let main_hand_item_name = main_hand_item.map_or("nothing", Item::name).to_string();
        let main_hand_item_sprite = main_hand_item.map(Item::looks_like).unwrap_or_default();
        let main_hand_item_color = main_hand_item.map(Item::color).unwrap_or_default();

        drop(units);
        drop(world);

        self.main_hand_display_label()
            .update(main_hand_item_name, ctx, window_size);
        self.main_hand_display_image()
            .set_sprite(main_hand_item_sprite);
        self.main_hand_display_image()
            .set_color(main_hand_item_color);
    }
}

impl SceneImpl for GameScene {
    fn on_update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::I) {
            return Some(Transition::Push(Scene::Inventory));
        }

        if input::is_mouse_scrolled_down(ctx)
            || input::is_key_with_mod_pressed(ctx, (Key::Z, KeyModifier::Shift))
        {
            self.world.borrow_mut().game_view.zoom.dec();
            self.need_redraw = true;
        } else if input::is_mouse_scrolled_up(ctx) || input::is_key_with_mod_pressed(ctx, Key::Z) {
            self.world.borrow_mut().game_view.zoom.inc();
            self.need_redraw = true;
        }

        if self.world.borrow().units().player().action.is_some() {
            self.make_world_tick(ctx);
            self.need_redraw = true;

            None
        } else {
            self.mode_update(ctx)
        }
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        if self.need_redraw || self.map_canvas.is_none() {
            self.map_canvas = Some(map_view::view::draw(
                ctx,
                &self.world,
                &self.assets,
                self.window_size,
                self.shift_of_view,
            ));
            self.need_redraw = false;
        }

        self.map_canvas.as_mut().unwrap().draw(ctx, Vec2::zero());
    }

    fn after_draw(&mut self, ctx: &mut Context) {
        // TODO: move this to UI

        map_view::view::draw_cursors(
            ctx,
            &self.world,
            &self.assets,
            self.window_size,
            self.cursors(),
        );
        map_view::ui::draw_log(ctx, &mut self.log);
        map_view::view::draw_unit(
            ctx,
            &self.assets.tileset,
            Vec2::new(5.0, 5.0),
            3.0,
            false,
            self.world.borrow().units().player_as_avatar(),
        );
        self.current_mode().borrow_mut().draw(ctx, self);
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
        self.need_redraw = true;
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
