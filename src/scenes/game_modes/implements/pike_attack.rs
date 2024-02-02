use std::time::Instant;

use geometry::{Direction, Point, Vec2};
use tetra::{
    input::{Key, KeyModifier, MouseButton},
    Context,
};

use crate::{
    colors::Colors,
    game::World,
    game::{actions::implements::Melee, traits::Name, AttackType},
    input::{self, Zero},
    lang::a,
    settings::Settings,
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    Cursor, CursorType, GameModeImpl,
};

pub struct PikeAttack {
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: u8,
    mouse_moved_pos: Point,
    shift_of_view: Point,
}

impl PikeAttack {
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

impl Default for PikeAttack {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for PikeAttack {
    fn cursors(&self, world: &World) -> Vec<Cursor> {
        let pos = self.shift_of_view + self.mouse_moved_pos;
        let damage = world
            .units()
            .player()
            .attack_damage(AttackType::Melee)
            .unwrap();
        let distance = (pos.distance(Point::default()).floor() - 1.0) as u8;
        let color = if distance <= damage.distance {
            Colors::LIGHT_CORAL
        } else {
            Colors::LIGHT_SKY_BLUE
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

        let r = damage.distance as i32 + 1;
        for i in -r..=r {
            for j in -r..=r {
                let point = Point::new(i, j);
                let distance = (point.distance(Point::default()).floor() - 1.0) as u8;
                if distance > damage.distance {
                    continue;
                }

                if !point.is_zero()
                    && !world
                        .map()
                        .get_tile(world.units().player().pos + point)
                        .units
                        .is_empty()
                {
                    cursors.push((point - self.shift_of_view, Colors::RED, CursorType::Select));
                }
            }
        }

        cursors
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        world.units().player().wield.main_hand().map_or(
            Err("You have nothing in your hands!".to_string()),
            |weapon| {
                if weapon.melee_damage().distance == 0 {
                    return Err(format!(
                        "You can't use distance attack with {}!",
                        a(weapon.name())
                    ));
                }

                Ok(())
            },
        )
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        self.update_mouse(ctx, game);
        if input::is_key_pressed(ctx, Key::Escape) {
            game.set_shift_of_view(Point::default());
            game.modes.pop();
            return None;
        } else if input::is_some_of_keys_pressed(ctx, &[Key::F, Key::Space, Key::Enter])
            || input::is_mouse_button_down(ctx, MouseButton::Left)
        {
            let pos = game.world.borrow().units().player().pos
                + game.shift_of_view()
                + self.mouse_moved_pos;
            game.try_start_action(Melee::new(pos).into());
            game.set_shift_of_view(Point::default());
            game.modes.pop();
            return None;
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let damage = game.world.borrow().units().player().melee_damage();
            let pos = self.shift_of_view + self.mouse_moved_pos + dir;
            let distance = (pos.distance(Point::default()).floor() - 1.0) as u8;
            if distance <= damage.distance {
                let now = Instant::now();
                if now.duration_since(self.last_shift).subsec_millis()
                    > Settings::instance().input.repeat_interval
                    || input::is_key_modifier_down(ctx, KeyModifier::Shift)
                {
                    self.last_shift = now;
                    game.set_shift_of_view(game.shift_of_view() + dir);
                }
            }
            game.try_rotate_player(Direction::from(game.shift_of_view() + self.mouse_moved_pos));
        }

        self.shift_of_view = game.shift_of_view();

        None
    }
}
