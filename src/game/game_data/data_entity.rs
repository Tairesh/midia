use serde::Deserialize;

use super::{item::ItemPrototype, names_pack::NamesPack};

#[allow(clippy::large_enum_variant)]
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
            assert_eq!(true, item.two_handed_tool);
            assert_eq!(None, item.wearable);
            assert!(item.melee_damage.is_some());
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
        static JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "shovel",
            "name": "Shovel",
            "looks_like": "shovel",
            "tags": [ "TOOL" ],
            "qualities": [
              "DIG"
            ],
            "mass": 2000,
            "two_handed_tool": true,
            "melee_damage": {
              "moves": 50,
              "damage": {
                "attribute": "STRENGTH",
                "dices": [
                  "D6"
                ],
                "modifier": -1
              }
            }
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
        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        check_shovel(&slice[0]);
        check_namepack(&slice[1]);
    }

    #[test]
    fn test_deserialize_shovel() {
        static JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "shovel",
            "name": "Shovel",
            "looks_like": "shovel",
            "tags": [ "TOOL" ],
            "qualities": [
              "DIG"
            ],
            "mass": 2000,
            "two_handed_tool": true,
            "melee_damage": {
              "moves": 50,
              "damage": {
                "attribute": "STRENGTH",
                "dices": [
                  "D6"
                ],
                "modifier": -1
              }
            }
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        check_shovel(&slice[0]);
    }

    #[test]
    fn test_deserialize_book() {
        static JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "book",
            "name": "Book",
            "looks_like": "book",
            "specials": [ "READABLE", "NAMED", "COLORED" ],
            "mass": 100
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
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
        static JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "cloak",
            "name": "cloak",
            "looks_like": "cloak",
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

        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
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

    #[test]
    fn test_deserialize_knife() {
        static JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "knife",
            "name": "knife",
            "looks_like": "knife",
            "tags": [
              "TOOL",
              "WEAPON"
            ],
            "qualities": [
              "BUTCH",
              "CUT"
            ],
            "mass": 100,
            "two_handed_tool": false,
            "melee_damage": {
              "moves": 10,
              "damage": {
                "attribute": "STRENGTH",
                "dices": [ "D4" ]
              }
            }
          }
      ]
      "#;

        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "knife");
            assert!(item.tags.contains(&ItemTag::Tool));
            assert!(item.tags.contains(&ItemTag::Weapon));
            assert!(item.qualities.contains(&ItemQuality::Butch));
            assert!(item.qualities.contains(&ItemQuality::Cut));
            assert!(item.melee_damage.is_some());
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }
}
