use geometry::Point;

use super::super::{
    super::map::{TerrainInteract, TerrainView},
    super::{
        game_data::DamageType,
        savage::{melee_attack_unit, melee_smash_terrain, TerrainAttackResult, UnitAttackResult},
        Action, Avatar, Item, LogEvent, World, Wound,
    },
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

const MELEE_ATTACK_MOVES: u32 = 10;

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct MeleeAttack {
    target: Point,
}

impl MeleeAttack {
    pub fn new(target: Point) -> Self {
        Self { target }
    }

    fn smash(self, action: &Action, world: &mut World) -> bool {
        let owner = action.owner(world);
        let weapon = owner.wield.active_hand();
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
                TerrainAttackResult::Miss => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} {} trying to smash the {} with {} {weapon_name} but miss{}!",
                            owner.name_for_actions(),
                            if owner.is_player() { "are" } else { "is" },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().2,
                            if owner.is_player() { "" } else { "es" },
                        ),
                        self.target,
                    ));
                }
                TerrainAttackResult::Hit(damage) => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} smash{} the {} with {} {weapon_name} for {damage} damage but didn't break it.",
                            owner.name_for_actions(),
                            if owner.is_player() { "" } else { "es" },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().2,
                        ),
                        self.target,
                    ));
                }
                TerrainAttackResult::Success(damage) => {
                    world.log().push(LogEvent::info(
                        format!(
                            "{} smash{} the {} with {} {weapon_name} for {damage} damage and completely destroyed it.",
                            owner.name_for_actions(),
                            if owner.is_player() { "" } else { "es" },
                            world.map().get_tile(self.target).terrain.name(),
                            owner.pronounce().2,
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
        let weapon_name = owner.wield.active_hand().map_or(
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
            let unit = world.get_unit(unit_id);
            let attack = melee_attack_unit(owner, unit);
            match attack {
                UnitAttackResult::Hit(damage) => {
                    let mut message = format!(
                        "{} attack{} {} with {} {weapon_name} and deal {} damage with {} penetration.",
                        owner.name_for_actions(),
                        if owner.is_player() { "" } else { "s" },
                        unit.name_for_actions(),
                        owner.pronounce().2,
                        damage.params.damage,
                        damage.params.penetration,
                    );
                    if damage.params.critical {
                        message.push_str(" Critical hit!");
                    }

                    if !damage.causes.shock && damage.causes.wounds.is_empty() {
                        message.push_str(" No effect.");
                    } else {
                        if damage.causes.shock {
                            message.push_str(
                                format!(" {} is stunned.", unit.name_for_actions()).as_str(),
                            );
                        }
                        if !damage.causes.wounds.is_empty() {
                            message.push_str(&format!(
                                " Attack causes wounds: {}",
                                damage
                                    .causes
                                    .wounds
                                    .iter()
                                    .copied()
                                    .map(Wound::name)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ));
                        }
                    }
                    world.log().push(LogEvent::success(message, self.target));

                    world.apply_damage(unit_id, damage);
                }
                UnitAttackResult::Miss => {
                    world.log().push(LogEvent::warning(
                        format!(
                            "{} attack{} {} with {} {weapon_name} and miss.",
                            owner.name_for_actions(),
                            if owner.is_player() { "" } else { "s" },
                            unit.name_for_actions(),
                            owner.pronounce().2,
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
        let weapon = owner.wield.active_hand().map_or(
            owner.personality.appearance.race.natural_weapon().0,
            Item::name,
        );

        world.log().push(LogEvent::info(
            format!(
                "{} swing{} in the air with {} {weapon}.",
                owner.name_for_actions(),
                if owner.is_player() { "" } else { "s" },
                owner.pronounce().2,
            ),
            self.target,
        ));
    }
}

impl ActionImpl for MeleeAttack {
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let distance = (actor.pos.distance(self.target).floor() - 1.0) as u8;
        let weapon = actor.wield.active_hand();
        if distance > 0 {
            let weapon_distance = weapon.map_or(0, |w| w.melee_damage().distance);
            if distance > weapon_distance {
                No("You can't reach the target from this distance".to_string())
            } else {
                Yes(MELEE_ATTACK_MOVES)
            }
        } else {
            Yes(MELEE_ATTACK_MOVES)
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

    use crate::game::map::items::helpers::{DEMONIC_SAP, STONE_AXE, STONE_KNIFE};
    use crate::game::map::terrains::Boulder;
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::{Action, Item};

    use super::{MeleeAttack, MELEE_ATTACK_MOVES};

    #[test]
    fn test_melee_attack_with_fists() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        add_npc(&mut world, Point::new(1, 0));
        world.player_mut().wield.clear();

        world.player_mut().action =
            Some(Action::new(0, MeleeAttack::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, 10);
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
        world.player_mut().wield.wield(Item::new(STONE_AXE));

        world.player_mut().action =
            Some(Action::new(0, MeleeAttack::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, MELEE_ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("with your stone axe"),
            "msg \"{}\" doesn't contains \"with your stone axe\"",
            event.msg
        );
    }

    #[test]
    fn test_smash_boulder() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world.player_mut().wield.wield(Item::new(DEMONIC_SAP));
        world.player_mut().action =
            Some(Action::new(0, MeleeAttack::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, MELEE_ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("smash the small boulder"),
            "msg \"{}\" doesn't contains \"smash the small boulder\"",
            event.msg
        );
    }

    #[test]
    fn test_cant_smash_with_knife() {
        let mut world = prepare_world();
        assert_eq!(world.meta.current_tick, 0);

        world.map().get_tile_mut(Point::new(1, 0)).terrain = Boulder::default().into();
        world.player_mut().wield.wield(Item::new(STONE_KNIFE));
        world.player_mut().action =
            Some(Action::new(0, MeleeAttack::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, MELEE_ATTACK_MOVES as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("swing in the air with your stone knife"),
            "msg \"{}\" doesn't contains \"swing in the air with your knife\"",
            event.msg
        );
    }
}
