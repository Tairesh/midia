use std::time::Instant;

use geometry::{Direction, Point, Vec2};
use tetra::{
    input::{Key, KeyModifier, MouseButton},
    Context,
};

use crate::{
    colors::Colors,
    game::World,
    game::{actions::implements::Shoot, traits::Name, AttackType, Fighter, RangedDistance},
    input,
    lang::a,
    settings::Settings,
};

use super::super::{
    super::{implements::GameScene, Transition},
    Cursor, CursorType, GameModeImpl,
};

pub struct Shooting {
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: u8,
    mouse_moved_pos: Point,
    shift_of_view: Point,
}

impl Shooting {
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

impl Default for Shooting {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Shooting {
    fn cursors(&self, world: &World) -> Option<Vec<Cursor>> {
        let pos = self.shift_of_view + self.mouse_moved_pos;
        let damage = world
            .units()
            .player()
            .weapon(AttackType::Shoot)
            .unwrap()
            .damage;
        let distance = RangedDistance::define(pos.distance_to(Point::default()), damage.distance);
        let color = match distance {
            RangedDistance::Close => Colors::LIME,
            RangedDistance::Medium => Colors::YELLOW,
            RangedDistance::Far => Colors::RED,
            RangedDistance::Melee | RangedDistance::Unreachable => Colors::LIGHT_SKY_BLUE,
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

        Some(cursors)
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        world.units().player().inventory.main_hand().map_or(
            Err("You have nothing in your hands!".to_string()),
            |item| {
                if item.ranged_damage().is_none() {
                    return Err(format!("You can't shoot from {}!", a(item.name())));
                }
                if let Some(need_ammo) = item.need_ammo() {
                    if !item.has_ammo(need_ammo.typ) {
                        return Err(format!("You have no ammo in {}!", a(item.name())));
                    }
                }

                Ok(())
            },
        )
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Option<Transition> {
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
            let action = Shoot::new(pos, &game.world.borrow());
            game.try_start_action(action);
            game.set_shift_of_view(Point::default());
            game.modes.pop();
            return None;
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let damage = game
                .world
                .borrow()
                .units()
                .player()
                .weapon(AttackType::Shoot)
                .unwrap()
                .damage;
            let pos = self.shift_of_view + self.mouse_moved_pos + dir;
            let distance =
                RangedDistance::define(pos.distance_to(Point::default()), damage.distance);
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
            game.try_rotate_player(Direction::from(game.shift_of_view() + self.mouse_moved_pos));
        }

        self.shift_of_view = game.shift_of_view();

        None
    }
}
