#![allow(dead_code)]

use geometry::Direction;
pub use tetra::{input::*, math::num_traits::Zero, Context};

#[derive(Debug, Copy, Clone)]
pub struct KeyWithMod {
    pub key: Key,
    pub key_mod: Option<KeyModifier>,
}

impl KeyWithMod {
    pub fn new(key: Key, key_mod: Option<KeyModifier>) -> Self {
        Self { key, key_mod }
    }

    pub fn key(key: Key) -> Self {
        Self::new(key, None)
    }

    pub fn with(mut self, key_mod: KeyModifier) -> Self {
        self.key_mod = Some(key_mod);
        self
    }
}

impl From<Key> for KeyWithMod {
    fn from(key: Key) -> Self {
        Self::key(key)
    }
}

impl From<(Key, KeyModifier)> for KeyWithMod {
    fn from((key, key_mod): (Key, KeyModifier)) -> Self {
        Self::key(key).with(key_mod)
    }
}

/// Check key is pressed and key mod is on
pub fn is_key_with_mod_pressed<K: Into<KeyWithMod>>(ctx: &mut Context, kwm: K) -> bool {
    let kwm: KeyWithMod = kwm.into();
    if !is_key_pressed(ctx, kwm.key) {
        return false;
    }
    if let Some(key_mod) = kwm.key_mod {
        is_key_modifier_down(ctx, key_mod)
    } else {
        is_no_key_modifiers(ctx)
    }
}

/// Nor Shift, nor Alt, nor Ctrl is pressed
pub fn is_no_key_modifiers(ctx: &Context) -> bool {
    is_key_modifier_up(ctx, KeyModifier::Shift)
        && is_key_modifier_up(ctx, KeyModifier::Alt)
        && is_key_modifier_up(ctx, KeyModifier::Ctrl)
}

/// Sum of downed keys that assumes direction
/// For example if `Key::Up` and `Key::Left` is pressed it will return `Some(Direction::NorthWest)`
pub fn get_direction_keys_down(ctx: &Context) -> Option<Direction> {
    let key_down = |np: Key, n: Key| -> bool {
        is_key_down(ctx, np) || (is_key_down(ctx, n) && is_key_modifier_up(ctx, KeyModifier::Shift))
    };
    if key_down(Key::NumPad5, Key::Num5) {
        return Some(Direction::Here);
    }
    if key_down(Key::NumPad7, Key::Num7) {
        return Some(Direction::NorthWest);
    }
    if key_down(Key::NumPad9, Key::Num9) {
        return Some(Direction::NorthEast);
    }
    if key_down(Key::NumPad3, Key::Num3) {
        return Some(Direction::SouthEast);
    }
    if key_down(Key::NumPad1, Key::Num1) {
        return Some(Direction::SouthWest);
    }
    let key_down = |k1: Key, k2: Key, n: Key| -> bool {
        is_key_down(ctx, k1)
            || is_key_down(ctx, k2)
            || (is_key_down(ctx, n) && is_key_modifier_up(ctx, KeyModifier::Shift))
    };
    let (mut moving_x, mut moving_y) = (0i8, 0i8);
    if key_down(Key::Up, Key::NumPad8, Key::Num8) {
        moving_y -= 1;
    }
    if key_down(Key::Down, Key::NumPad2, Key::Num2) {
        moving_y += 1;
    }
    if key_down(Key::Left, Key::NumPad4, Key::Num4) {
        moving_x -= 1;
    }
    if key_down(Key::Right, Key::NumPad6, Key::Num6) {
        moving_x += 1;
    }
    match (moving_x, moving_y) {
        (-1, -1) => Some(Direction::NorthWest),
        (-1, 1) => Some(Direction::SouthWest),
        (1, -1) => Some(Direction::NorthEast),
        (1, 1) => Some(Direction::SouthEast),
        (1, 0) => Some(Direction::East),
        (-1, 0) => Some(Direction::West),
        (0, -1) => Some(Direction::North),
        (0, 1) => Some(Direction::South),
        _ => None,
    }
}

/// Mouse was scrolled up or down (or even left or right)
pub fn is_mouse_scrolled(ctx: &mut Context) -> bool {
    !get_mouse_wheel_movement(ctx).is_zero()
}
