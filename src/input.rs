#![allow(dead_code)]

use geometry::Direction;
use serde::{Deserializer, Serializer};
pub use tetra::{input::*, math::num_traits::Zero, Context};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyWithMod {
    pub key: Key,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl serde::Serialize for KeyWithMod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = format!("{:?}", self.key);
        if self.alt {
            s = format!("Alt+{s}");
        }
        if self.ctrl {
            s = format!("Ctrl+{s}");
        }
        if self.shift {
            s = format!("Shift+{s}");
        }
        serializer.serialize_str(&s)
    }
}

impl<'de> serde::Deserialize<'de> for KeyWithMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let shift = str.contains("Shift+");
        let ctrl = str.contains("Ctrl+");
        let alt = str.contains("Alt+");
        let key = format!(
            "\"{}\"",
            str.replace("Shift+", "")
                .replace("Ctrl+", "")
                .replace("Alt+", "")
        );
        let key = serde_json::from_str(&key).map_err(serde::de::Error::custom)?;

        Ok(Self {
            key,
            shift,
            ctrl,
            alt,
        })
    }
}

impl KeyWithMod {
    pub fn new(key: Key, shift: bool, ctrl: bool, alt: bool) -> Self {
        Self {
            key,
            shift,
            ctrl,
            alt,
        }
    }

    pub fn from_key(key: Key) -> Self {
        Self {
            key,
            shift: false,
            ctrl: false,
            alt: false,
        }
    }

    pub fn shift(key: Key) -> Self {
        Self {
            key,
            shift: true,
            ctrl: false,
            alt: false,
        }
    }

    pub fn ctrl(key: Key) -> Self {
        Self {
            key,
            shift: false,
            ctrl: true,
            alt: false,
        }
    }

    pub fn alt(key: Key) -> Self {
        Self {
            key,
            shift: false,
            ctrl: false,
            alt: true,
        }
    }
}

impl From<Key> for KeyWithMod {
    fn from(key: Key) -> Self {
        Self::from_key(key)
    }
}

impl From<(Key, KeyModifier)> for KeyWithMod {
    fn from((key, key_mod): (Key, KeyModifier)) -> Self {
        match key_mod {
            KeyModifier::Shift => Self {
                key,
                shift: true,
                ctrl: false,
                alt: false,
            },
            KeyModifier::Ctrl => Self {
                key,
                shift: false,
                ctrl: true,
                alt: false,
            },
            KeyModifier::Alt => Self {
                key,
                shift: false,
                ctrl: false,
                alt: true,
            },
            _ => unreachable!("Only Shift, Ctrl and Alt are supported"),
        }
    }
}

/// Check key is pressed and key mod is on
pub fn is_key_with_mod_pressed<K: Into<KeyWithMod>>(ctx: &mut Context, kwm: K) -> bool {
    let kwm: KeyWithMod = kwm.into();
    if !is_key_pressed(ctx, kwm.key) {
        return false;
    }
    if kwm.shift && !is_key_modifier_down(ctx, KeyModifier::Shift) {
        return false;
    }
    if kwm.ctrl && !is_key_modifier_down(ctx, KeyModifier::Ctrl) {
        return false;
    }
    if kwm.alt && !is_key_modifier_down(ctx, KeyModifier::Alt) {
        return false;
    }

    true
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

pub fn is_some_of_keys_pressed(ctx: &mut Context, keys: &[Key]) -> bool {
    keys.iter().any(|&k| is_key_pressed(ctx, k))
}

pub fn get_key_with_mod_pressed(ctx: &mut Context) -> Vec<KeyWithMod> {
    let mut keys = Vec::new();
    for &key in get_keys_pressed(ctx) {
        keys.push(KeyWithMod::new(
            key,
            is_key_modifier_down(ctx, KeyModifier::Shift),
            is_key_modifier_down(ctx, KeyModifier::Ctrl),
            is_key_modifier_down(ctx, KeyModifier::Alt),
        ));
    }

    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(KeyWithMod::new(Key::A, false, true, false), "\"Ctrl+A\"")]
    #[test_case(KeyWithMod::from_key(Key::A), "\"A\"")]
    #[test_case(KeyWithMod::shift(Key::A), "\"Shift+A\"")]
    #[test_case(KeyWithMod::new(Key::Delete, false, true, true), "\"Ctrl+Alt+Delete\"")]
    fn test_serializing_key_with_mod(kwm: KeyWithMod, expected: &str) {
        let serialized = serde_json::to_string(&kwm).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test_case("\"Ctrl+A\"", KeyWithMod::new(Key::A, false, true, false))]
    #[test_case("\"A\"", KeyWithMod::from_key(Key::A))]
    #[test_case("\"Shift+A\"", KeyWithMod::shift(Key::A))]
    #[test_case("\"Ctrl+Alt+Delete\"", KeyWithMod::new(Key::Delete, false, true, true))]
    fn test_deserializing_key_with_mod(serialized: &str, expected: KeyWithMod) {
        let kwm: KeyWithMod = serde_json::from_str(serialized).unwrap();
        assert_eq!(kwm, expected);
    }
}
