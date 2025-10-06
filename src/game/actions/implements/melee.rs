use roguemetry::Point;

use super::super::{
    super::map::{TerrainInteract, TerrainView},
    super::{
        log::helpers::unit_attack_success,
        savage::{
            melee_attack_unit, melee_smash_terrain, DamageType, TerrainMeleeAttackResult,
            UnitMeleeAttackResult, ATTACK_MOVES,
        },
        traits::Name,
        Action, AttackType, Avatar, LogEvent, World,
    },
    ActionImpl,
    ActionPossibility::{self, No, Yes},
    ActionType, AttackTarget,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Melee {
    target: AttackTarget,
}

impl Melee {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pos: Point, world: &World) -> ActionType {
        Self {
            target: AttackTarget::auto(pos, world),
        }
        .into()
    }

    #[cfg(test)]
    pub fn target(&self) -> AttackTarget {
        self.target
    }

    fn smash(self, action: &Action, world: &mut World) -> bool {
        let AttackTarget::Terrain(target) = self.target else {
            return false;
        };
        let owner = action.owner(world);
        let weapon = owner.as_fighter().weapon(AttackType::Melee).unwrap();
        let can_smash = weapon.damage.damage_types.contains(&DamageType::Blunt)
            && world.map().get_tile(target).terrain.smash().is_some();
        if !can_smash {
            return false;
        }
        let attack = melee_smash_terrain(owner.as_fighter(), &world.map().get_tile(target).terrain);
        match attack {
            TerrainMeleeAttackResult::Miss => {
                world.log().push(LogEvent::info(
                    format!(
                        "{} tr{} to break the {} with {} {} but miss{}!",
                        owner.name_for_actions(),
                        if owner.pronouns().verb_ends_with_s() {
                            "ies"
                        } else {
                            "y"
                        },
                        world.map().get_tile(target).terrain.name(),
                        owner.pronouns().possessive_adjective(),
                        weapon.name,
                        if owner.pronouns().verb_ends_with_s() {
                            "es"
                        } else {
                            ""
                        },
                    ),
                    target,
                ));
            }
            TerrainMeleeAttackResult::Hit(damage) => {
                world.log().push(LogEvent::info(
                    format!(
                        "{} hit{} the {} with {} {} dealing {damage} damage but it didn't break it.",
                        owner.name_for_actions(),
                        if owner.pronouns().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        world.map().get_tile(target).terrain.name(),
                        owner.pronouns().possessive_adjective(),
                        weapon.name,
                    ),
                    target,
                ));
            }
            TerrainMeleeAttackResult::Success(damage) => {
                world.log().push(LogEvent::info(
                    format!(
                        "{} hit{} the {} with {} {} dealing {damage} damage and completely destroying it.",
                        owner.name_for_actions(),
                        if owner.pronouns().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        world.map().get_tile(target).terrain.name(),
                        owner.pronouns().possessive_adjective(),
                        weapon.name,
                    ),
                    target,
                ));
                let (new_terrain, mut items) =
                    world.map().get_tile(target).terrain.smash().unwrap().result;
                world.map().get_tile_mut(target).terrain = new_terrain;
                world.map().get_tile_mut(target).items.append(&mut items);
            }
        }

        true
    }

    fn attack_unit(self, action: &Action, world: &mut World) -> bool {
        let AttackTarget::Avatar(unit_id) = self.target else {
            return false;
        };
        let owner = action.owner(world);
        let weapon_name = owner.as_fighter().weapon(AttackType::Melee).unwrap().name;
        let unit = world.units.get_unit(unit_id);
        let attack = melee_attack_unit(owner.as_fighter(), unit.as_fighter());
        match attack {
            UnitMeleeAttackResult::Hit(hit) => {
                let damage = hit.params.damage.to_string();
                for event in unit_attack_success(
                    owner,
                    unit,
                    &hit,
                    format!(
                        "{} attack{} {} with {} {weapon_name} dealing {} damage{}.",
                        owner.name_for_actions(),
                        if owner.pronouns().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        unit.name_for_actions(),
                        owner.pronouns().possessive_adjective(),
                        if hit.params.damage == 0 {
                            "no"
                        } else {
                            &damage
                        },
                        if hit.params.penetration > 0 {
                            format!(" and {} armor penetration", hit.params.penetration)
                        } else {
                            String::new()
                        }
                    ),
                ) {
                    world.log().push(event);
                }

                world.apply_damage(unit_id, hit);
            }
            UnitMeleeAttackResult::Miss => {
                world.log().push(LogEvent::warning(
                    format!(
                        "{} attack{} {} with {} {weapon_name} but miss{}.",
                        owner.name_for_actions(),
                        if owner.pronouns().verb_ends_with_s() {
                            "s"
                        } else {
                            ""
                        },
                        unit.name_for_actions(),
                        owner.pronouns().possessive_adjective(),
                        if owner.pronouns().verb_ends_with_s() {
                            "es"
                        } else {
                            ""
                        },
                    ),
                    unit.pos(),
                ));
            }
        }

        true
    }

    fn swing(action: &Action, world: &mut World) {
        let owner = action.owner(world);
        let weapon = owner.as_fighter().weapon(AttackType::Melee).unwrap().name;

        world.log().push(LogEvent::info(
            format!(
                "{} wave{} {} {weapon} in the air.",
                owner.name_for_actions(),
                if owner.pronouns().verb_ends_with_s() {
                    "s"
                } else {
                    ""
                },
                owner.pronouns().possessive_adjective(),
            ),
            owner.pos(),
        ));
    }
}

impl ActionImpl for Melee {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let actor = world.units.get_unit(actor_id);
        if actor.char_sheet().shock {
            return No("You are in shock".to_string());
        }

        let target = match self.target {
            AttackTarget::Avatar(id) => world.units.get_unit(id).pos(),
            AttackTarget::Terrain(pos) => pos,
        };
        let distance = (actor.pos().distance_to(target).floor() - 1.0) as u8;

        let weapon = actor.as_fighter().weapon(AttackType::Melee);
        if weapon.is_none() {
            return No("You have no weapon".to_string());
        }

        if distance > 0 {
            let weapon_distance = weapon.unwrap().damage.distance;
            if distance > weapon_distance {
                No("You can't reach the target from this distance".to_string())
            } else {
                Yes(ATTACK_MOVES)
            }
        } else {
            Yes(ATTACK_MOVES)
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        // TODO: attack with cutting weapon
        if !self.attack_unit(action, world) && !self.smash(action, world) {
            Self::swing(action, world);
        }
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::Point;

    use crate::game::map::items::helpers::{DEMONIC_SAP, GOD_AXE, STONE_KNIFE, STONE_SPEAR};
    use crate::game::map::terrains::Boulder;
    use crate::game::world::tests::{add_dummy, prepare_world};
    use crate::game::{Action, Avatar, Item, Race};

    use super::{Melee, ATTACK_MOVES};

    #[test]
    fn test_melee_attack_with_fists() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_dummy(&mut world, Point::new(1, 0));
        world.units.player_mut().inventory_mut().unwrap().clear();

        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("with your fists"),
            "msg \"{}\" doesn't contains \"with your fists\"",
            event.msg
        );
    }

    #[test]
    fn test_melee_attack_with_weapon() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_dummy(&mut world, Point::new(1, 0));
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(GOD_AXE));

        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("with your god axe"),
            "msg \"{}\" doesn't contains \"with your god axe\"",
            event.msg
        );
    }

    #[test]
    fn test_smash_boulder() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(DEMONIC_SAP));
        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event
                .msg
                .contains("the small boulder with your demonic metal sap"),
            "msg \"{}\" doesn't contains \"the small boulder with your demonic metal sap\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_smash_with_knife() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(STONE_KNIFE));
        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("wave your stone knife in the air"),
            "msg \"{}\" doesn't contains \"wave your stone knife in the air\"",
            event.msg
        );
    }

    #[test]
    fn test_smash_with_fists() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);
        assert_eq!(
            world.units.player().personality.appearance.race,
            Race::Gazan
        );

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world.units.player_mut().inventory_mut().unwrap().clear();
        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("the small boulder with your fists"),
            "msg \"{}\" doesn't contains \"the small boulder with your fists\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_smash_with_fangs() {
        let mut world = prepare_world();
        world.units.player_mut().personality.appearance.race = Race::Lagnam;
        world.units.player_mut().inventory_mut().unwrap().clear();

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        let action = Action::new(0, Melee::new(Point::new(1, 0), &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("wave your fangs in the air"),
            "msg \"{}\" doesn't contains \"wave your fangs in the air\"",
            event.msg
        );
    }

    #[test]
    fn test_spear_attack() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(STONE_SPEAR));

        let target = Point::new(2, 0);
        add_dummy(&mut world, target);
        let action = Action::new(0, Melee::new(target, &world).into(), &world).unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("You attack Dummy with your stone spear"),
            "msg \"{}\" doesn't contains \"You attack Dummy with your stone spear\"",
            event.msg
        );
    }

    #[test]
    fn test_spear_cant_attack_on_distance_3() {
        let mut world = prepare_world();
        world
            .units
            .player_mut()
            .inventory_mut()
            .unwrap()
            .wield(Item::new(STONE_SPEAR));

        let target = Point::new(3, 0);
        assert!(Action::new(0, Melee::new(target, &world).into(), &world).is_err());
    }
}
