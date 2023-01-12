use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{
    super::Avatar,
    items::{Axe, BodyPart, Cloak, Corpse, Gravestone, Hat, Knife, Rags, Shovel},
};

// TODO: JSON-ize all items

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    Shovel,
    Axe,
    Knife,
    Corpse,
    Gravestone,
    BodyPart,
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
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum ItemTag {
    Dig,
    Butch,
}

#[cfg(test)]
mod tests {
    use super::super::super::map::terrains::GraveData;
    use super::{
        super::{
            super::{
                bodies::{Freshness, OrganData},
                races::{helpers::body, tests::personality::dead_boy},
            },
            items::{Axe, BodyPart, BodyPartType, Cloak, Corpse, Gravestone, Hat, Knife, Shovel},
        },
        Item, ItemInteract, ItemTag, ItemView,
    };

    #[test]
    fn test_shovel() {
        let shovel: Item = Shovel::new().into();
        assert_eq!("shovel", shovel.name());
        assert!(shovel.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_axe() {
        let axe: Item = Axe::new().into();
        assert_eq!("axe", axe.name());
        assert!(axe.tags().contains(&ItemTag::Butch));
        assert!(!axe.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_knife() {
        let knife: Item = Knife::new().into();
        assert_eq!("knife", knife.name());
        assert!(knife.tags().contains(&ItemTag::Butch));
        assert!(!knife.tags().contains(&ItemTag::Dig));
    }

    #[test]
    fn test_corpse() {
        let character = dead_boy();
        let body = body(OrganData::new(&character, Freshness::Rotten));
        let corpse: Item = Corpse::new(character, body).into();
        assert_eq!("naked rotten gazan boy corpse", corpse.name());
    }

    #[test]
    fn test_bodypart() {
        let character = dead_boy();
        let brain: Item = BodyPart::new(
            "brain",
            BodyPartType::Brain,
            OrganData::new(&character, Freshness::Fresh),
        )
        .into();
        // TODO: We can't see the sex of the brain but can understand that is the child one
        assert_eq!("fresh gazan boy brain", brain.name());

        let head: Item = BodyPart::new(
            "head",
            BodyPartType::Head,
            OrganData::new(&character, Freshness::Skeletal),
        )
        .into();
        assert_eq!("gazan boy skull", head.name());
    }

    #[test]
    fn test_gravestone() {
        let character = dead_boy();
        let gravestone: Item = Gravestone::new(GraveData {
            character,
            death_year: 255,
        })
        .into();
        assert_eq!("gravestone", gravestone.name());
        assert!(gravestone.is_readable());
        assert!(gravestone.read().contains("Dead Boy"));
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
