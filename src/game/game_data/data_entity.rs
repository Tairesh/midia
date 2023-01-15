use serde::Deserialize;

use super::{item::ItemPrototype, names_pack::NamesPack};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DataEntity {
    Item(ItemPrototype),
    // TODO: terrains
    NamesPack(NamesPack),
}

#[cfg(test)]
mod tests {
    use crate::game::races::{BodySlot, Race, Sex};

    use super::super::item::{ItemQuality, ItemSpecial, ItemTag, WearLayer};
    use super::DataEntity;

    fn check_shovel(shovel: &DataEntity) {
        if let DataEntity::Item(item) = shovel {
            assert_eq!("shovel", item.id);
            assert_eq!("Shovel", item.name);
            assert_eq!(2000, item.mass);
            assert_eq!(1, item.tags.len());
            assert!(item.tags.contains(&ItemTag::Tool));
            assert_eq!(1, item.qualities.len());
            assert!(item.qualities.contains(&ItemQuality::Dig));
            assert_eq!(0, item.specials.len());
        } else {
            panic!("Expected DataEntity::Item, got {:?}", shovel);
        }
    }

    fn check_namepack(names_pack: &DataEntity) {
        if let DataEntity::NamesPack(names_pack) = names_pack {
            assert_eq!("core", names_pack.id);
            assert!(names_pack.names.contains_key(&Race::Gazan));
            if let Some(names) = names_pack.names.get(&Race::Gazan) {
                assert!(names.contains_key(&Sex::Male));
                if let Some(names) = names.get(&Sex::Male) {
                    assert!(names.contains(&"Dragan".to_string()));
                } else {
                    panic!("Expected names!");
                }
            } else {
                panic!("Expected Gazan names, got {:?}", names_pack);
            }
        } else {
            panic!("Expected DataEntity::NamesPack, got {:?}", names_pack);
        }
    }

    #[test]
    fn test_deserialize_different_types() {
        let json = r#"
        [
          {
            "type": "item",
            "id": "shovel",
            "name": "Shovel",
            "look_like": "shovel",
            "tags": [ "TOOL" ],
            "qualities": [
              "DIG"
            ],
            "mass": 2000,
            "two_handed_tool": true
          },
          {
            "type": "names_pack",
            "id": "core",
            "names": {
                "gazan": {
                    "m": [ "Dragan", "Yasma" ],
                    "f": [ "Dooka" ],
                    "u": []
                },
                "lagnam": {
                    "m": [ "Grem" ],
                    "f": [ "Test" ],
                    "u": []
                },
                "nyarnik": {
                    "m": [ "Mnerk" ],
                    "f": [ "Shasha" ],
                    "u": []
                },
                "totik": {
                    "m": [],
                    "f": [],
                    "u": []
                }
            }
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        check_shovel(&slice[0]);
        check_namepack(&slice[1]);
    }

    #[test]
    fn test_deserialize_shovel() {
        let json = r#"
        [
          {
            "type": "item",
            "id": "shovel",
            "name": "Shovel",
            "look_like": "shovel",
            "tags": [ "TOOL" ],
            "qualities": [
              "DIG"
            ],
            "mass": 2000,
            "two_handed_tool": true
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        check_shovel(&slice[0]);
    }

    #[test]
    fn test_deserialize_book() {
        let json = r#"
        [
          {
            "type": "item",
            "id": "book",
            "name": "Book",
            "look_like": "book",
            "specials": [ "READABLE", "NAMED", "COLORED" ],
            "mass": 100
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "book");
            assert!(item.specials.contains(&ItemSpecial::Readable));
            assert!(item.specials.contains(&ItemSpecial::Named));
            assert!(item.specials.contains(&ItemSpecial::Colored));
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }

    #[test]
    fn test_deserialize_cloak() {
        let json = r#"
        [
          {
            "type": "item",
            "id": "cloak",
            "name": "cloak",
            "look_like": "cloak",
            "mass": 100,
            "wearable": [
              [ "torso", "OUTER", 1 ],
              [ "left_arm", "OUTER", 1 ],
              [ "right_arm", "OUTER", 1 ],
              [ "left_leg", "OUTER", 1 ],
              [ "right_leg", "OUTER", 1 ]
            ]
          }
        ]
        "#;

        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "cloak");
            assert!(item.wearable.is_some());
            if let Some(wearable) = &item.wearable {
                assert_eq!(wearable.len(), 5);
                assert!(wearable.contains(&(BodySlot::Torso, WearLayer::Outer, 1)));
                assert!(wearable.contains(&(BodySlot::LeftArm, WearLayer::Outer, 1)));
                assert!(wearable.contains(&(BodySlot::RightArm, WearLayer::Outer, 1)));
                assert!(wearable.contains(&(BodySlot::LeftLeg, WearLayer::Outer, 1)));
                assert!(wearable.contains(&(BodySlot::RightLeg, WearLayer::Outer, 1)));
            } else {
                panic!("Expected wearable!");
            }
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }
}
