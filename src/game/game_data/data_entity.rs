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
    use crate::game::game_data::item::AmmoType;
    use crate::game::game_data::DamageType;
    use crate::game::races::{BodySlot, Race, Sex};
    use crate::game::savage::DamageDice;
    use crate::game::Attribute;

    use super::super::item::{ItemQuality, ItemSize, ItemTag, Material, WearLayer};
    use super::DataEntity;

    fn check_shovel(shovel: &DataEntity) {
        if let DataEntity::Item(item) = shovel {
            assert_eq!("shovel_stone", item.id);
            assert_eq!("stone shovel", item.name);
            assert_eq!(ItemSize::Medium, item.size);
            assert_eq!(1, item.tags.len());
            assert!(item.tags.contains(&ItemTag::Tool));
            assert_eq!(1, item.qualities.len());
            assert!(item.qualities.contains(&ItemQuality::Dig));
            assert_eq!(true, item.two_handed_tool);
            assert!(item.wearable.is_none());
            assert!(item.melee_damage.is_some());
            assert_eq!(2, item.materials.len());
            assert!(item.materials.contains(&Material::Wood));
            assert!(item.materials.contains(&Material::Stone));
        } else {
            panic!("Expected DataEntity::Item, got {:?}", shovel);
        }
    }

    fn check_namepack(names_pack: &DataEntity) {
        if let DataEntity::NamesPack(names_pack) = names_pack {
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
        const JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "shovel_stone",
            "name": "stone shovel",
            "looks_like": "shovel_stone",
            "tags": ["TOOL"],
            "qualities": ["DIG"],
            "size": "MEDIUM",
            "two_handed_tool": true,
            "materials": ["wood", "stone"],
            "melee_damage": {
              "damage": {
                "attribute": "STRENGTH",
                "dices": ["D6"],
                "modifier": -1
              },
              "damage_types": ["BLUNT"]
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
        const JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "shovel_stone",
            "name": "stone shovel",
            "looks_like": "shovel_stone",
            "tags": ["TOOL"],
            "qualities": ["DIG"],
            "size": "MEDIUM",
            "two_handed_tool": true,
            "materials": ["wood", "stone"],
            "melee_damage": {
              "damage": {
                "attribute": "STRENGTH",
                "dices": ["D6"],
                "modifier": -1
              },
              "damage_types": ["BLUNT"]
            }
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        check_shovel(&slice[0]);
    }

    #[test]
    fn test_deserialize_cloak() {
        const JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "cloak",
            "name": "cloak",
            "looks_like": "cloak",
            "size": "SMALL",
            "materials": ["cloth"],
            "wearable": {
              "layer": "OUTER",
              "armor": 1,
              "variants": [
                ["torso", "left_arm", "right_arm", "left_leg", "right_leg"]
              ]
            },
            "color_from_material": "cloth"
          }
        ]
        "#;

        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "cloak");
            assert!(item.wearable.is_some());
            assert_eq!(1, item.materials.len());
            assert!(item.materials.contains(&Material::Cloth));
            assert!(item.color_from_material.is_some());
            assert_eq!(item.color_from_material.unwrap(), Material::Cloth);
            if let Some(wearable) = &item.wearable {
                assert_eq!(wearable.armor, 1);
                assert_eq!(wearable.layer, WearLayer::Outer);
                assert_eq!(wearable.variants.len(), 1);
                if let Some(variant) = wearable.variants.iter().next() {
                    assert_eq!(variant.len(), 5);
                    assert!(variant.contains(&BodySlot::Torso));
                    assert!(variant.contains(&BodySlot::LeftArm));
                    assert!(variant.contains(&BodySlot::RightArm));
                    assert!(variant.contains(&BodySlot::LeftLeg));
                    assert!(variant.contains(&BodySlot::RightLeg));
                } else {
                    panic!("Expected variant!");
                }
            } else {
                panic!("Expected wearable!");
            }
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }

    #[test]
    fn test_deserialize_knife() {
        const JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "knife_stone",
            "name": "stone knife",
            "looks_like": "knife_stone",
            "tags": ["TOOL", "WEAPON"],
            "qualities": ["BUTCH", "CUT"],
            "size": "SMALL",
            "two_handed_tool": false,
            "materials": ["wood", "stone"],
            "melee_damage": {
              "damage": {
                "attribute": "STRENGTH",
                "dices": ["D4"]
              },
              "damage_types": ["SLASH", "PIERCE"]
            }
          }
      ]
      "#;

        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "knife_stone");
            assert!(item.tags.contains(&ItemTag::Tool));
            assert!(item.tags.contains(&ItemTag::Weapon));
            assert!(item.qualities.contains(&ItemQuality::Butch));
            assert!(item.qualities.contains(&ItemQuality::Cut));
            assert!(item.melee_damage.is_some());
            if let Some(melee_damage) = &item.melee_damage {
                assert_eq!(melee_damage.damage.attribute, Some(Attribute::Strength));
                assert_eq!(melee_damage.damage.dices.len(), 1);
                assert_eq!(melee_damage.damage.dices[0], DamageDice::D4);
                assert_eq!(melee_damage.damage.modifier, 0);
                assert_eq!(melee_damage.damage_types.len(), 2);
                assert!(melee_damage.damage_types.contains(&DamageType::Slash));
                assert!(melee_damage.damage_types.contains(&DamageType::Pierce));
            } else {
                panic!("Expected melee_damage!");
            }
            assert_eq!(item.materials.len(), 2);
            assert!(item.materials.contains(&Material::Wood));
            assert!(item.materials.contains(&Material::Stone));
            assert_eq!(item.size, ItemSize::Small);
            assert!(!item.two_handed_tool);
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }

    #[test]
    fn test_deserialize_bow_and_arrow() {
        const JSON: &str = r#"
        [
          {
            "type": "item",
            "id": "shortbow_wood",
            "name": "wooden short bow",
            "description": "Emphasizing portability and agility over power, this all-wood bow is suitable for small game and harassing enemies.",
            "looks_like": "bow_wood",
            "size": "MEDIUM",
            "materials": ["wood"],
            "tags": ["WEAPON"],
            "two_handed_tool": true,
            "ranged_damage": {
              "damage": {
                "attribute": "STRENGTH",
                "dices": ["D4"]
              },
              "damage_types": ["PIERCE"],
              "distance": 12
            },
            "ammo_types": ["ARROW"]
          },
          {
            "type": "item",
            "id": "arrow_wood",
            "name": "wooden arrow",
            "description": "Weakest arrow, made entirely from wood.",
            "looks_like": "arrow_wood",
            "size": "SMALL",
            "materials": ["wood"],
            "ammo": {
              "typ": ["ARROW"],
              "damage_modifier": {
                "damage": -1
              }
            }
          }
      ]
      "#;

        let data: Vec<DataEntity> = serde_json::from_str(JSON).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!(item.id, "shortbow_wood");
            assert!(!item.tags.contains(&ItemTag::Tool));
            assert!(item.tags.contains(&ItemTag::Weapon));
            assert!(item.qualities.is_empty());
            assert!(item.ranged_damage.is_some());
            if let Some(ranged_damage) = &item.ranged_damage {
                assert_eq!(ranged_damage.damage.attribute, Some(Attribute::Strength));
                assert_eq!(ranged_damage.damage.dices.len(), 1);
                assert_eq!(ranged_damage.damage.dices[0], DamageDice::D4);
                assert_eq!(ranged_damage.damage.modifier, 0);
                assert_eq!(ranged_damage.damage_types.len(), 1);
                assert!(ranged_damage.damage_types.contains(&DamageType::Pierce));
            } else {
                panic!("Expected ranged_damage!");
            }
            assert_eq!(item.materials.len(), 1);
            assert!(item.materials.contains(&Material::Wood));
            assert_eq!(item.size, ItemSize::Medium);
            assert!(item.two_handed_tool);
            assert_eq!(item.ammo_types.len(), 1);
            assert!(item.ammo_types.contains(&AmmoType::Arrow));
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
        assert!(matches!(slice[1], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[1] {
            assert_eq!(item.id, "arrow_wood");
            assert!(!item.tags.contains(&ItemTag::Tool));
            assert!(!item.tags.contains(&ItemTag::Weapon));
            assert!(item.qualities.is_empty());
            assert!(item.ranged_damage.is_none());
            if let Some(ammo_value) = &item.ammo {
                assert_eq!(ammo_value.typ.len(), 1);
                assert!(ammo_value.typ.contains(&AmmoType::Arrow));
                assert!(ammo_value.damage_modifier.damage_dice.is_none());
                assert_eq!(ammo_value.damage_modifier.damage, -1);
                assert_eq!(ammo_value.damage_modifier.penetration, 0);
            } else {
                panic!("Expected ammo_value!");
            }
            assert_eq!(item.materials.len(), 1);
            assert!(item.materials.contains(&Material::Wood));
            assert_eq!(item.size, ItemSize::Small);
            assert!(!&item.two_handed_tool);
            assert!(item.ammo_types.is_empty());
        } else {
            panic!("Expected DataEntity::Item, got {:?}", slice[0]);
        }
    }
}
