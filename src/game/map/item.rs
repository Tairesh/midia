use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{
    super::Avatar,
    items::{Axe, Cloak, Hat, Knife, Rags, Shovel},
};

// TODO: JSON-ize all items

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Shovel,
    Axe,
    Knife,
    Hat,
    Cloak,
    Rags,
}

#[enum_dispatch(Item)]
pub trait ItemView {
    fn name(&self) -> String;
    fn looks_like(&self) -> &'static str;
}

#[enum_dispatch(Item)]
pub trait ItemInteract {
    fn tags(&self) -> HashSet<ItemTag> {
        HashSet::new()
    }
    fn qualities(&self) -> HashSet<ItemQuality> {
        HashSet::new()
    }
    fn mass(&self) -> u32;
    // in grams
    fn wield_time(&self, _avatar: &Avatar) -> f64 {
        // 100 grams per tick
        self.mass() as f64 / 100.0
    }
    fn drop_time(&self, _avatar: &Avatar) -> f64 {
        10.0
    }
    // TODO: same as TerrainInteract
    fn is_readable(&self) -> bool {
        false
    }
    fn read(&self) -> String {
        unreachable!()
    }
    fn is_wearable(&self) -> bool {
        false
    }
    fn tool_or_weapon(&self) -> bool {
        self.tags().contains(&ItemTag::Tool) || self.tags().contains(&ItemTag::Weapon)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ItemTag {
    Tool,
    Weapon,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum ItemQuality {
    Dig,
    Butch,
}

#[cfg(test)]
mod tests {
    use super::{
        super::items::{Axe, Cloak, Hat, Knife, Shovel},
        Item, ItemInteract, ItemQuality, ItemTag, ItemView,
    };

    #[test]
    fn test_shovel() {
        let shovel: Item = Shovel::new().into();
        assert_eq!("shovel", shovel.name());
        assert!(shovel.tags().contains(&ItemTag::Tool));
        assert!(shovel.qualities().contains(&ItemQuality::Dig));
    }

    #[test]
    fn test_axe() {
        let axe: Item = Axe::new().into();
        assert_eq!("axe", axe.name());
        assert!(axe.tags().contains(&ItemTag::Tool));
        assert!(axe.tags().contains(&ItemTag::Weapon));
        assert!(axe.qualities().contains(&ItemQuality::Butch));
        assert!(!axe.qualities().contains(&ItemQuality::Dig));
    }

    #[test]
    fn test_knife() {
        let knife: Item = Knife::new().into();
        assert_eq!("knife", knife.name());
        assert!(knife.tags().contains(&ItemTag::Tool));
        assert!(knife.tags().contains(&ItemTag::Weapon));
        assert!(knife.qualities().contains(&ItemQuality::Butch));
        assert!(!knife.qualities().contains(&ItemQuality::Dig));
    }

    #[test]
    fn test_hat() {
        let hat: Item = Hat::new().into();
        assert_eq!("hat", hat.name());
        assert!(hat.is_wearable());
    }

    #[test]
    fn test_cloak() {
        let cloak: Item = Cloak::new().into();
        assert_eq!("cloak", cloak.name());
        assert!(cloak.is_wearable());
    }
}
