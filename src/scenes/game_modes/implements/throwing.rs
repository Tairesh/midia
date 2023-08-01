use std::time::Instant;

use geometry::{Point, Vec2};
use tetra::{
    input::{Key, KeyModifier},
    Context,
};

use crate::game::actions::implements::Throw;
use crate::game::RangedDistance;
use crate::{colors::Colors, game::World, input, settings::Settings};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    Cursor, CursorType, GameModeImpl,
};

pub struct Throwing {
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: u8,
    mouse_moved_pos: Point,
    shift_of_view: Point,
}

impl Throwing {
    pub fn new() -> Self {
        Self {
            last_shift: Instant::now(),
            last_mouse_position: Vec2::zero(),
            mouse_moved: false,
            last_zoom: 0,
            mouse_moved_pos: Point::default(),
            shift_of_view: Point::default(),
        }
    }

    fn update_mouse(&mut self, ctx: &mut Context, game: &mut GameScene) {
        let mouse = input::get_mouse_position(ctx);
        let zoom_view = game.world.borrow().game_view.zoom.as_view();
        let zoom = game.world.borrow().game_view.zoom.0;
        if mouse != self.last_mouse_position || zoom != self.last_zoom {
            self.last_mouse_position = mouse;
            self.last_zoom = zoom;
            if self.mouse_moved {
                let (w, h) = game.window_size;
                self.mouse_moved_pos = ((mouse - Vec2::new((w / 2) as f32, (h / 2) as f32))
                    / (game.assets.tileset.tile_size as f32 * zoom_view))
                    .into();
            }
            self.mouse_moved = true;
        }
    }
}

impl Default for Throwing {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Throwing {
    fn cursors(&self, world: &World) -> Vec<Cursor> {
        let pos = self.shift_of_view + self.mouse_moved_pos;
        let damage = world
            .player()
            .wield
            .active_hand()
            .unwrap()
            .throw_damage()
            .unwrap();
        let distance = RangedDistance::define(pos.distance(Point::default()), damage.distance);
        let color = match distance {
            RangedDistance::Melee => Colors::ORANGE,
            RangedDistance::Close => Colors::LIME,
            RangedDistance::Medium => Colors::YELLOW,
            RangedDistance::Far => Colors::RED,
            RangedDistance::Unreachable => Colors::LIGHT_SKY_BLUE,
        };

        let mut cursors: Vec<Cursor> = self
            .mouse_moved_pos
            .line_to(-self.shift_of_view)
            .into_iter()
            .skip(1)
            .map(|p| (p, color.with_alpha(0.2), CursorType::Fill))
            .collect();

        cursors.push((
            self.mouse_moved_pos,
            color.with_alpha(1.0),
            CursorType::Select,
        ));

        cursors
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        world.player().wield.active_hand().map_or(
            Err("You have nothing in your hands!".to_string()),
            |item| {
                item.throw_damage()
                    .map_or(Err(format!("You can't throw {}!", item.name())), |_| Ok(()))
            },
        )
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        self.update_mouse(ctx, game);
        if input::is_key_pressed(ctx, Key::Escape) {
            game.set_shift_of_view(Point::default());
            game.modes.pop();
            return None;
        } else if input::is_some_of_keys_pressed(ctx, &[Key::T, Key::Space, Key::Enter]) {
            let pos =
                game.world.borrow().player().pos + game.shift_of_view() + self.mouse_moved_pos;
            let unit_in_tile = game
                .world
                .borrow()
                .map()
                .get_tile(pos)
                .units
                .iter()
                .copied()
                .next();
            if let Some(unit_id) = unit_in_tile {
                game.try_start_action(Throw::new(unit_id).into());
            } else {
                // TODO: throw to terrain
                let item = game
                    .world
                    .borrow_mut()
                    .player_mut()
                    .wield
                    .take_from_active_hand()
                    .unwrap();
                game.world.borrow().map().get_tile_mut(pos).items.push(item);
            }
            game.set_shift_of_view(Point::default());
            game.modes.pop();
            return None;
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let damage = game
                .world
                .borrow()
                .player()
                .wield
                .active_hand()
                .unwrap()
                .throw_damage()
                .unwrap();
            let pos = self.shift_of_view + self.mouse_moved_pos + dir;
            let distance = RangedDistance::define(pos.distance(Point::default()), damage.distance);
            if distance != RangedDistance::Unreachable {
                let now = Instant::now();
                if now.duration_since(self.last_shift).subsec_millis()
                    > Settings::instance().input.repeat_interval
                    || input::is_key_modifier_down(ctx, KeyModifier::Shift)
                {
                    self.last_shift = now;
                    game.set_shift_of_view(game.shift_of_view() + dir);
                }
            }
        }

        self.shift_of_view = game.shift_of_view();

        None
    }
}