#![allow(dead_code)]

use geometry::Direction;
use serde::{Deserializer, Serializer};
pub use tetra::{input::*, math::num_traits::Zero, Context};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyWithMod {
    pub key: Key,
    pub key_mod: Option<KeyModifier>,
}

impl serde::Serialize for KeyWithMod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.key_mod {
            Some(key_mod) => serializer.serialize_str(&format!("{}+{:?}", key_mod, self.key)),
            None => serializer.serialize_str(&format!("{:?}", self.key)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for KeyWithMod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        let parts: Vec<String> = str
            .split('+')
            .map(|s| format!("\"{}\"", s.trim()))
            .collect();
        match parts.len() {
            1 => Ok(Self::key(
                serde_json::from_str(&parts[0]).map_err(serde::de::Error::custom)?,
            )),
            2 => Ok(Self::new(
                serde_json::from_str(&parts[1]).map_err(serde::de::Error::custom)?,
                serde_json::from_str(&parts[0]).map_err(serde::de::Error::custom)?,
            )),
            _ => Err(serde::de::Error::custom("Invalid key")),
        }
    }
}

impl KeyWithMod {
    pub fn new(key: Key, key_mod: KeyModifier) -> Self {
        Self {
            key,
            key_mod: Some(key_mod),
        }
    }

    pub fn key(key: Key) -> Self {
        Self { key, key_mod: None }
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

pub fn is_some_of_keys_pressed(ctx: &mut Context, keys: &[Key]) -> bool {
    keys.iter().any(|&k| is_key_pressed(ctx, k))
}

pub fn get_key_with_mod_pressed(ctx: &mut Context) -> Vec<KeyWithMod> {
    let mut keys = Vec::new();
    for &key in get_keys_pressed(ctx) {
        if is_no_key_modifiers(ctx) {
            keys.push(KeyWithMod::key(key));
        } else {
            if is_key_modifier_down(ctx, KeyModifier::Shift) {
                keys.push(KeyWithMod::new(key, KeyModifier::Shift));
            }
            if is_key_modifier_down(ctx, KeyModifier::Alt) {
                keys.push(KeyWithMod::new(key, KeyModifier::Alt));
            }
            if is_key_modifier_down(ctx, KeyModifier::Ctrl) {
                keys.push(KeyWithMod::new(key, KeyModifier::Ctrl));
            }
        }
    }

    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializing_key_with_mod() {
        let kwm = KeyWithMod::new(Key::A, KeyModifier::Ctrl);
        let serialized = serde_json::to_string(&kwm).unwrap();
        assert_eq!(serialized, "\"Ctrl+A\"");

        let kwm = KeyWithMod::key(Key::A);
        let serialized = serde_json::to_string(&kwm).unwrap();
        assert_eq!(serialized, "\"A\"");
    }

    #[test]
    fn test_deserializing_key_with_mod() {
        let kwm: KeyWithMod = serde_json::from_str("\"Ctrl+A\"").unwrap();
        assert_eq!(kwm, KeyWithMod::new(Key::A, KeyModifier::Ctrl));

        let kwm: KeyWithMod = serde_json::from_str("\"A\"").unwrap();
        assert_eq!(kwm, KeyWithMod::key(Key::A));
    }
}
