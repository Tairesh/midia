#![allow(dead_code)]

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tetra::graphics::Color;

use crate::game::game_data::AmmoType;
use crate::game::{
    AttackType, DamageValue, GameData, ItemPrototype, ItemQuality, ItemSize, ItemTag,
};

use super::container::Container;

const CUSTOM_PROTO: &str = "custom";

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    #[serde(rename = "p")]
    proto: String,
    #[serde(default, rename = "cp")]
    custom_proto: Option<ItemPrototype>,
    #[serde(default, rename = "n")]
    named: Option<String>,
    #[serde(default, rename = "c")]
    colored: Option<Color>,
    #[serde(default, rename = "r")]
    readable: Option<String>,
    #[serde(default, rename = "l")]
    looks_like: Option<String>,
    #[serde(default, rename = "t")]
    container: Option<Container>,
}

impl Item {
    // TODO: create a builder for this
    pub fn new(proto: impl Into<String>) -> Self {
        let mut item = Self {
            proto: proto.into(),
            custom_proto: None,
            named: None,
            colored: None,
            readable: None,
            looks_like: None,
            container: None,
        };
        if let Some(material) = item.proto().color_from_material {
            item.colored = Some(material.into());
        }

        let container = item
            .proto()
            .qualities
            .iter()
            .find(|q| matches!(q, ItemQuality::Container { .. }))
            .cloned();
        if let Some(ItemQuality::Container { volume, for_ammo }) = container {
            item = item.with_container([], volume, for_ammo);
        }

        item
    }

    pub fn custom(proto: ItemPrototype) -> Self {
        Self {
            proto: CUSTOM_PROTO.to_string(),
            named: None,
            colored: proto.color_from_material.map(Color::from),
            readable: None,
            looks_like: None,
            container: None,
            custom_proto: Some(proto),
        }
    }

    pub fn proto(&self) -> &ItemPrototype {
        if let Some(custom_proto) = &self.custom_proto {
            custom_proto
        } else {
            let game_data = GameData::instance();
            game_data.get_item_prototype(&self.proto)
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

    pub fn with_container(
        mut self,
        items: impl Into<Vec<Item>>,
        volume: u8,
        for_ammo: impl Into<HashSet<AmmoType>>,
    ) -> Self {
        self.container = Some(Container::new(items, volume, for_ammo));
        self
    }

    pub fn with_items_inside(mut self, items: impl Into<Vec<Item>>) -> Self {
        if let Some(container) = &mut self.container {
            container.push_items(items);
        } else {
            unreachable!("Trying to put items into not-container item");
        }

        self
    }

    pub fn name(&self) -> &str {
        if let Some(named) = &self.named {
            return named;
        }

        &self.proto().name
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

        &self.proto().looks_like
    }

    pub fn container(&self) -> Option<&Container> {
        self.container.as_ref()
    }

    pub fn container_mut(&mut self) -> Option<&mut Container> {
        self.container.as_mut()
    }

    pub fn qualities(&self) -> &Vec<ItemQuality> {
        &self.proto().qualities
    }

    pub fn tags(&self) -> &HashSet<ItemTag> {
        &self.proto().tags
    }

    pub fn size(&self) -> ItemSize {
        self.proto().size
    }

    pub fn is_two_handed(&self) -> bool {
        self.proto().two_handed_tool || self.size() > ItemSize::Medium
    }

    pub fn is_tool(&self) -> bool {
        self.tags().contains(&ItemTag::Tool)
    }

    pub fn is_weapon(&self) -> bool {
        self.tags().contains(&ItemTag::Weapon)
    }

    /// Used for determining if an item should be displayed in full size when wielded.
    pub fn tool_or_weapon(&self) -> bool {
        self.is_tool() || self.is_weapon()
    }

    pub fn is_wearable(&self) -> bool {
        self.proto().wearable.is_some()
    }

    pub fn armor(&self) -> u8 {
        self.proto().wearable.as_ref().map_or(0, |w| w.armor)
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

    pub fn damage(&self, attack_type: AttackType) -> Option<DamageValue> {
        match attack_type {
            AttackType::Melee => {
                // TODO: check for minimum strength
                if self.proto().melee_damage.is_some() {
                    return self.proto().melee_damage.clone();
                }

                Some(DamageValue::improvised_melee(self))
            }
            AttackType::Throw => {
                if self.proto().throw_damage.is_some() {
                    return self.proto().throw_damage.clone();
                }

                DamageValue::improvised_throw(self)
            }
            AttackType::Shoot => self.proto().ranged_damage.clone(),
        }
    }

    pub fn melee_damage(&self) -> DamageValue {
        self.damage(AttackType::Melee).unwrap()
    }

    pub fn throw_damage(&self) -> Option<DamageValue> {
        self.damage(AttackType::Throw)
    }

    pub fn ranged_damage(&self) -> Option<DamageValue> {
        self.damage(AttackType::Shoot)
    }

    pub fn ammo_types(&self) -> &HashSet<AmmoType> {
        &self.proto().ammo_types
    }

    pub fn is_ammo(&self, ammo_type: AmmoType) -> bool {
        if let Some(ammo) = &self.proto().ammo {
            ammo.typ.contains(&ammo_type)
        } else {
            false
        }
    }

    pub fn has_ammo(&self, ammo_type: AmmoType) -> bool {
        if self.is_ammo(ammo_type) {
            return true;
        }

        if let Some(container) = &self.container {
            container.items.iter().any(|item| item.has_ammo(ammo_type))
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::map::items::helpers::{BACKPACK, GOD_AXE, QUIVER, WOODEN_ARROW};

    use super::Item;

    #[test]
    fn test_backpack() {
        let mut backpack = Item::new(BACKPACK);
        assert_eq!(backpack.name(), "leather backpack");
        if let Some(container) = backpack.container_mut() {
            assert_eq!(container.max_volume, 30);
            assert!(!container.is_for_ammo());
            assert_eq!(container.volume_used(), 0);
            let axe = Item::new(GOD_AXE);
            container.push_item(axe);
            assert_eq!(container.volume_used(), 1);
        } else {
            panic!("backpack is not a container");
        }
    }

    #[test]
    fn test_quiver() {
        let mut quiver = Item::new(QUIVER);
        assert_eq!(quiver.name(), "leather quiver");
        if let Some(container) = quiver.container_mut() {
            assert_eq!(container.max_volume, 15);
            assert!(container.is_for_ammo());
            assert_eq!(container.volume_used(), 0);
            let axe = Item::new(GOD_AXE);
            container.push_item(axe);
            assert_eq!(container.volume_used(), 0);
            let arrow = Item::new(WOODEN_ARROW);
            container.push_item(arrow);
            assert_eq!(container.volume_used(), 1);
        } else {
            panic!("backpack is not a container");
        }
    }
}
