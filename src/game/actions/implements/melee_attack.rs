use geometry::Point;

use super::super::{
    super::{melee_attack, Action, AttackResult, Avatar, LogEvent, World, Wound},
    ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct MeleeAttack {
    target: Point,
}

impl MeleeAttack {
    pub fn new(target: Point) -> Self {
        Self { target }
    }
}

impl ActionImpl for MeleeAttack {
    fn is_possible(&self, actor: &Avatar, _world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let distance = (actor.pos.distance(self.target).floor() - 1.0) as u8;
        let weapon = actor.wield.active_hand();
        let weapon_moves = weapon.map_or(10, |w| w.melee_damage().moves);
        if distance > 0 {
            let weapon_distance = weapon.map_or(0, |w| w.melee_damage().distance);
            if distance > weapon_distance {
                No("You can't reach the target from this distance".to_string())
            } else {
                Yes(weapon_moves as u32)
            }
        } else {
            Yes(weapon_moves as u32)
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        // TODO: refactor this to smaller functions
        let owner = action.owner(world);
        let weapon_name = if let Some(weapon) = owner.wield.active_hand() {
            weapon.name()
        } else {
            "fists"
        };
        let unit_id = world
            .map()
            .get_tile(self.target)
            .units
            .iter()
            .copied()
            .next();
        if let Some(unit_id) = unit_id {
            let unit = world.get_unit(unit_id);
            let attack = melee_attack(owner, unit);
            match attack {
                AttackResult::Hit(damage) => {
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
                AttackResult::Miss => {
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
        } else {
            // TODO: attack terrains and items
            world.log().push(LogEvent::info(
                format!(
                    "{} swing{} in the air with {} {weapon_name}.",
                    owner.name_for_actions(),
                    if owner.is_player() { "" } else { "s" },
                    owner.pronounce().2,
                ),
                self.target,
            ));
        };
    }
}

#[cfg(test)]
mod tests {
    use geometry::Point;

    use crate::game::map::items::helpers::axe;
    use crate::game::world::tests::{add_npc, prepare_world};
    use crate::game::Action;

    use super::MeleeAttack;

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
        world.player_mut().wield.wield(axe());

        world.player_mut().action =
            Some(Action::new(0, MeleeAttack::new(Point::new(1, 0)).into(), &world).unwrap());
        world.tick();

        assert_eq!(world.meta.current_tick, axe().melee_damage().moves as u128);
        let mut log = world.log();
        let event = &log.new_events()[0];
        assert!(
            event.msg.contains("with your axe"),
            "msg \"{}\" doesn't contains \"with your axe\"",
            event.msg
        );
    }
}
