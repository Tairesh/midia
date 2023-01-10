use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

use geometry::{Direction, Point, TwoDimDirection, Vec2};
use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        DrawParams, Rectangle,
    },
    Context,
};

use crate::{
    app::App,
    assets::{Assets, Tileset},
    colors::Colors,
    game::{
        map::{item::ItemView, terrain::TerrainView},
        Action, ActionType, World,
    },
    ui::{GameLog, Label, Position, SomeUISprites, SomeUISpritesMut, UiSprite, Vertical},
};

use super::super::{
    game_modes::{implements::Walking, GameMode, GameModeImpl},
    SceneImpl, SomeTransitions,
};

pub struct GameScene {
    sprites: [Box<dyn UiSprite>; 2],
    pub world: Rc<RefCell<World>>,
    pub modes: Vec<Rc<RefCell<GameMode>>>,
    pub cursor: Mesh,
    pub log: GameLog,
    pub shift_of_view: Point,
    pub assets: Rc<Assets>,
    pub window_size: (i32, i32),
}

impl GameScene {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let world = app.get_world();
        let name_label = Box::new(Label::new(
            world.borrow().player().person().mind.name.as_str(),
            app.assets.fonts.header.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(50.0, 1.0),
        ));
        let current_time_label = Box::new(Label::new(
            format!("{}", world.borrow().meta.current_tick),
            app.assets.fonts.default.clone(),
            Colors::WHITE_SMOKE,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 5.0 }),
        ));
        Self {
            sprites: [name_label, current_time_label],
            modes: vec![Rc::new(RefCell::new(Walking::new().into()))],
            cursor: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(
                    0.0,
                    0.0,
                    app.assets.tileset.tile_size as f32,
                    app.assets.tileset.tile_size as f32,
                ),
            )
            .unwrap(),
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
        let window_size = self.window_size;
        self.current_time_label()
            .update(current_time, ctx, window_size);
    }

    fn current_time_label(&mut self) -> &mut Label {
        self.sprites[1].as_label().unwrap()
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
        tetra::graphics::clear(ctx, Colors::BLACK);
        let world = self.world.borrow();
        let width = self.window_size.0 as f32;
        let height = self.window_size.1 as f32;
        let zoom = world.game_view.zoom;
        let scale = zoom.as_scale();
        let zoom = zoom.as_view();
        let tile_size = self.tile_size();
        let window_size_in_tiles = (
            (width / tile_size).ceil() as i32,
            (height / tile_size).ceil() as i32,
        );
        let center = Vec2::new(
            width / 2.0 - tile_size / 2.0,
            height / 2.0 - tile_size / 2.0,
        );
        let center_tile = world.player().pos + self.shift_of_view;
        let left_top = center_tile + (-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
        let right_bottom = center_tile + (window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
        world.map().load_tiles_between(left_top, right_bottom);
        for (pos, tile) in world.map().tiles_between(left_top, right_bottom) {
            if !world.is_visible(pos) {
                continue; // TODO: TileView struct for remembering tiles and optimizing drawing
            }
            let dx = pos.x - center_tile.x;
            let dy = pos.y - center_tile.y;
            let this_tile_size = Tileset::get_size(tile.terrain.looks_like());
            let asset_tile_size = self.assets.tileset.tile_size as f32;
            let x_correction = -(this_tile_size.x - asset_tile_size) / 2.0 * zoom;
            let y_correction = -(this_tile_size.y - asset_tile_size) * zoom;
            let params = DrawParams::new()
                .position(Vec2::new(
                    (center.x + dx as f32 * tile_size + x_correction).round(),
                    (center.y + dy as f32 * tile_size + y_correction).round(),
                ))
                .scale(scale);
            self.assets
                .tileset
                .draw_region(ctx, tile.terrain.looks_like(), params.clone());
            if let Some(item) = tile.top_item() {
                self.assets
                    .tileset
                    .draw_region(ctx, item.looks_like(), params.clone());
                if tile.items.len() > 1 {
                    self.assets.tileset.draw_region(ctx, "highlight", params);
                }
            }
            // TODO: multitile units
            let position = Vec2::new(
                center.x + dx as f32 * tile_size,
                center.y + dy as f32 * tile_size,
            );
            for i in tile.units.iter().copied() {
                let unit = world.get_unit(i);
                unit.draw(ctx, &self.assets.tileset, position, zoom, true);
            }
        }
        // if world.player().action.is_some() {
        //     self.draw_action_loader(ctx, center);
        // } else {
        //     self.action_text = None;
        // }
        for (delta, color) in self.current_mode().borrow().cursors(&world) {
            let delta = delta * tile_size;
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .position(center + delta)
                    .scale(scale)
                    .color(color.with_alpha(0.7)),
            );
        }

        for (i, msg) in self.log.texts.iter_mut().enumerate() {
            msg.draw(Vec2::new(10.0, height - 20.0 * (i + 1) as f32), ctx);
        }
    }

    fn after_draw(&mut self, ctx: &mut Context) {
        // UI
        self.world.borrow().player().draw(
            ctx,
            &self.assets.tileset,
            Vec2::new(5.0, 5.0),
            3.0,
            false,
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
