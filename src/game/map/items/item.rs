#![allow(dead_code)]

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::game::game_data::{ItemPrototype, ItemQuality, ItemTag};

use super::specials::{Colored, Container, LookLike, Named, Readable};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub proto: ItemPrototype,
    named: Option<Named>,
    colored: Option<Colored>,
    readable: Option<Readable>,
    look_like: Option<LookLike>,
    container: Option<Container>,
}

impl Item {
    pub fn new(proto: ItemPrototype) -> Self {
        Self {
            proto,
            named: None,
            colored: None,
            readable: None,
            look_like: None,
            container: None,
        }
    }

    pub fn with_named(mut self, name: impl Into<String>) -> Self {
        self.named = Some(Named { name: name.into() });
        self
    }

    pub fn with_colored(mut self, color: impl Into<Color>) -> Self {
        self.colored = Some(Colored {
            color: color.into(),
        });
        self
    }

    pub fn with_readable(mut self, text: impl Into<String>) -> Self {
        self.readable = Some(Readable { text: text.into() });
        self
    }

    pub fn with_look_like(mut self, look_like: impl Into<String>) -> Self {
        self.look_like = Some(LookLike {
            look_like: look_like.into(),
        });
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
            return &named.name;
        }

        &self.proto.name
    }

    pub fn color(&self) -> Color {
        if let Some(colored) = &self.colored {
            return colored.color;
        }

        Color::WHITE
    }

    pub fn read(&self) -> Option<&str> {
        if let Some(readable) = &self.readable {
            return Some(&readable.text);
        }

        None
    }

    pub fn look_like(&self) -> &str {
        if let Some(look_like) = &self.look_like {
            return &look_like.look_like;
        }

        &self.proto.look_like
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

    pub fn mass(&self) -> u32 {
        self.proto.mass
    }

    pub fn two_handed(&self) -> bool {
        self.proto.two_handed_tool || self.mass() > 5000
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
        self.proto.is_wearable
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
        // 1000 grams per tick
        self.mass() as f32 / 1000.0
    }

    pub fn wield_time(&self) -> f32 {
        // 100 grams per tick
        self.mass() as f32 / 100.0
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