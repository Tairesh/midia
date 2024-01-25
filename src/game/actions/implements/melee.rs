use geometry::Point;

use super::super::{
    super::map::{TerrainInteract, TerrainView},
    super::{
        super::lang::a,
        game_data::DamageType,
        log::helpers::unit_attack_success,
        savage::{
            melee_attack_unit, melee_smash_terrain, TerrainMeleeAttackResult,
            UnitMeleeAttackResult, ATTACK_MOVES,
        },
        traits::Name,
        Action, AttackType, Avatar, Item, LogEvent, World,
    },
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Melee {
    target: Point,
}

impl Melee {
    pub fn new(target: Point) -> Self {
        Self { target }
    }

    fn smash(self, action: &Action, world: &mut World) -> bool {
        let owner = action.owner(world);
        let weapon = owner.wield.main_hand();
        let (weapon_name, can_smash) = if let Some(weapon) = weapon {
            (
                weapon.name(),
                weapon
                    .melee_damage()
                    .damage_types
                    .contains(&DamageType::Blunt),
            )
        } else {
            let (name, damage) = owner.personality.appearance.race.natural_weapon();
            (name, damage.damage_types.contains(&DamageType::Blunt))
        };

        if can_smash && world.map().get_tile(self.target).terrain.is_smashable() {
            let attack = melee_smash_terrain(owner, &world.map().get_tile(self.target).terrain);
            match attack {
                TerrainMeleeAttackResult::Miss => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} tr{} to break the {} with {} {weapon_name} but miss{}!",
                            owner.name_for_actions(),
                            if owner.pronounce().verb_ends_with_s() {
                                "ies"
                            } else {
                                "y"
                            },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().possessive_adjective(),
                            if owner.pronounce().verb_ends_with_s() {
                                "es"
                            } else {
                                ""
                            },
                        ),
                        self.target,
                    ));
                }
                TerrainMeleeAttackResult::Hit(damage) => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} hit{} the {} with {} {weapon_name} dealing {damage} damage but it didn't break it.",
                            owner.name_for_actions(),
                            if owner.pronounce().verb_ends_with_s() {
                                "s"
                            } else {
                                ""
                            },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().possessive_adjective(),
                        ),
                        self.target,
                    ));
                }
                TerrainMeleeAttackResult::Success(damage) => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} hit{} the {} with {} {weapon_name} dealing {damage} damage and completely destroying it.",
                            owner.name_for_actions(),
                            if owner.pronounce().verb_ends_with_s() {
                                "s"
                            } else {
                                ""
                            },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().possessive_adjective(),
                        ),
                        self.target,
                    ));
                    let (new_terrain, mut items) =
                        world.map().get_tile(self.target).terrain.smash_result();
                    world.map().get_tile_mut(self.target).terrain = new_terrain;
                    world
                        .map()
                        .get_tile_mut(self.target)
                        .items
                        .append(&mut items);
                }
            }

            true
        } else {
            false
        }
    }

    fn attack_unit(self, action: &Action, world: &mut World) -> bool {
        let owner = action.owner(world);
        let weapon_name = owner.wield.main_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );
        let unit_id = world
            .map()
            .get_tile(self.target)
            .units
            .iter()
            .copied()
            .next();
        if let Some(unit_id) = unit_id {
            let unit = world.units.get_unit(unit_id);
            let attack = melee_attack_unit(owner, unit);
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
                            if owner.pronounce().verb_ends_with_s() {
                                "s"
                            } else {
                                ""
                            },
                            unit.name_for_actions(),
                            owner.pronounce().possessive_adjective(),
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
                            if owner.pronounce().verb_ends_with_s() {
                                "s"
                            } else {
                                ""
                            },
                            unit.name_for_actions(),
                            owner.pronounce().possessive_adjective(),
                            if owner.pronounce().verb_ends_with_s() {
                                "es"
                            } else {
                                ""
                            },
                        ),
                        self.target,
                    ));
                }
            }

            true
        } else {
            false
        }
    }

    fn swing(self, action: &Action, world: &mut World) {
        let owner = action.owner(world);
        let weapon = owner.wield.main_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        world.log().push(LogEvent::info(
            format!(
                "{} wave{} {} {weapon} in the air.",
                owner.name_for_actions(),
                if owner.pronounce().verb_ends_with_s() {
                    "s"
                } else {
                    ""
                },
                owner.pronounce().possessive_adjective(),
            ),
            self.target,
        ));
    }
}

impl ActionImpl for Melee {
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let distance = (actor.pos.distance(self.target).floor() - 1.0) as u8;
        let weapon = actor.wield.main_hand();
        if distance > 0 {
            let weapon_distance = weapon.map_or(0, |w| w.melee_damage().distance);
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
            self.swing(action, world);
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::map::items::helpers::{DEMONIC_SAP, GOD_AXE, STONE_KNIFE};
    use crate::game::map::terrains::Boulder;
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item, Race};

    use super::{Melee, ATTACK_MOVES};

    #[test]
    fn test_melee_attack_with_fists() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(1, 0));
        world.units.player_mut().wield.clear();

        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
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

        add_npc(&mut world, Point::new(1, 0));
        world.units.player_mut().wield.wield(Item::new(GOD_AXE));

        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
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
        world.units.player_mut().wield.wield(Item::new(DEMONIC_SAP));
        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
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
        world.units.player_mut().wield.wield(Item::new(STONE_KNIFE));
        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
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
        world.units.player_mut().wield.clear();
        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("the small boulder with your fists"),
            "msg \"{}\" doesn't contains \"fists\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_smash_with_fangs() {
        let mut world = prepare_world();
        world.units.player_mut().personality.appearance.race = Race::Lagnam;
        world.units.player_mut().wield.clear();

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world.units.player_mut().action =
            Some(Action::new(0, Melee::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("wave your fangs in the air"),
            "msg \"{}\" doesn't contains \"swing in the air with your fangs\"",
            event.msg
        );
    }
}
