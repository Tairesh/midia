#![allow(dead_code)]

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::game::{ItemPrototype, ItemQuality, ItemSize, ItemTag, MeleeDamageValue};

use super::container::Container;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub proto: ItemPrototype,
    named: Option<String>,
    colored: Option<Color>,
    readable: Option<String>,
    looks_like: Option<String>,
    container: Option<Container>,
}

impl Item {
    pub fn new(proto: ItemPrototype) -> Self {
        Self {
            proto,
            named: None,
            colored: None,
            readable: None,
            looks_like: None,
            container: None,
        }
    }

    pub fn with_named(mut self, name: impl Into<String>) -> Self {
        self.named = Some(name.into());
        self
    }

    pub fn with_colored(mut self, color: impl Into<Color>) -> Self {
        self.colored = Some(color.into());
        self
    }

    pub fn with_readable(mut self, text: impl Into<String>) -> Self {
        self.readable = Some(text.into());
        self
    }

    pub fn with_looks_like(mut self, looks_like: impl Into<String>) -> Self {
        self.looks_like = Some(looks_like.into());
        self
    }

    pub fn with_container(mut self, items: impl Into<Vec<Item>>) -> Self {
        self.container = Some(Container {
            items: items.into(),
        });
        self
    }

    pub fn name(&self) -> &str {
        if let Some(named) = &self.named {
            return named;
        }

        &self.proto.name
    }

    pub fn color(&self) -> Color {
        if let &Some(color) = &self.colored {
            return color;
        }

        Color::WHITE
    }

    pub fn read(&self) -> Option<&str> {
        if let Some(readable) = &self.readable {
            return Some(readable);
        }

        None
    }

    pub fn looks_like(&self) -> &str {
        if let Some(looks_like) = &self.looks_like {
            return looks_like;
        }

        &self.proto.looks_like
    }

    pub fn container(&mut self) -> Option<&mut Container> {
        self.container.as_mut()
    }

    pub fn qualities(&self) -> &HashSet<ItemQuality> {
        &self.proto.qualities
    }

    pub fn tags(&self) -> &HashSet<ItemTag> {
        &self.proto.tags
    }

    pub fn size(&self) -> ItemSize {
        self.proto.size
    }

    pub fn is_two_handed(&self) -> bool {
        self.proto.two_handed_tool || self.size() >= ItemSize::Medium
    }

    pub fn is_tool(&self) -> bool {
        self.tags().contains(&ItemTag::Tool)
    }

    pub fn is_weapon(&self) -> bool {
        self.tags().contains(&ItemTag::Weapon)
    }

    pub fn tool_or_weapon(&self) -> bool {
        self.is_tool() || self.is_weapon()
    }

    pub fn is_wearable(&self) -> bool {
        self.proto.wearable.is_some()
    }

    pub fn armor(&self) -> u8 {
        self.proto.wearable.as_ref().map_or(0, |w| w.armor)
    }

    pub fn is_readable(&self) -> bool {
        self.readable.is_some()
    }

    pub fn is_book(&self) -> bool {
        self.tags().contains(&ItemTag::Book)
    }

    pub fn is_container(&self) -> bool {
        self.container.is_some()
    }

    pub fn drop_time(&self) -> f32 {
        match self.size() {
            ItemSize::Huge => 10.0,
            _ => 1.0,
        }
    }

    pub fn wield_time(&self) -> f32 {
        match self.size() {
            ItemSize::Tiny => 1.0,
            ItemSize::Small => 5.0,
            ItemSize::Medium => 10.0,
            ItemSize::Large => 20.0,
            ItemSize::Huge => 50.0,
        }
    }

    pub fn melee_damage(&self) -> MeleeDamageValue {
        // TODO: check for minimum strength
        if let Some(damage) = &self.proto.melee_damage {
            return damage.clone();
        }

        MeleeDamageValue::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{axe, backpack};

    #[test]
    fn test_backpack() {
        let mut backpack = backpack();
        assert_eq!(backpack.name(), "leather backpack");
        if let Some(container) = backpack.container() {
            assert_eq!(container.items.len(), 0);
            let axe = axe();
            container.items.push(axe);
            assert_eq!(container.items.len(), 1);
        } else {
            panic!("backpack is not a container");
        }
    }
}
