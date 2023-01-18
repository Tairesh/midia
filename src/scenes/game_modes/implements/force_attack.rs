use geometry::{Point, DIR8};
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use crate::colors::Colors;
use crate::game::{melee_attack, AttackResult, World, Wound};
use crate::input;
use crate::scenes::implements::GameScene;
use crate::scenes::SomeTransitions;

use super::super::GameModeImpl;

pub struct ForceAttack {
    pub target: Option<Point>,
}

impl ForceAttack {
    pub fn new() -> Self {
        Self { target: None }
    }
}

impl GameModeImpl for ForceAttack {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(target) = self.target {
            vec![(target - world.player().pos, Colors::LIGHT_CORAL)]
        } else {
            DIR8.iter()
                .copied()
                .filter(|&dir| {
                    !world
                        .map()
                        .get_tile(world.player().pos + dir)
                        .units
                        .is_empty()
                })
                .map(|dir| (dir.into(), Colors::RED))
                .collect()
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        // TODO: select targets on distance, with two modes, as in CoQ
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.target = Some(game.world.borrow().player().pos + dir);
            game.try_rotate_player(dir);
        } else if let Some(target) = self.target {
            // TODO start action with moves cost
            let world = game.world.borrow();
            let player = world.player();
            let weapon_name = if let Some(weapon) = player.wield.active_hand() {
                weapon.name()
            } else {
                "fists"
            };
            let hit_result = if let Some(&unit_id) =
                world.map().get_tile(target).units.iter().next()
            {
                let unit = world.get_unit(unit_id);
                let attack = melee_attack(player, unit);
                match attack {
                    AttackResult::Hit(damage) => {
                        let mut message = format!(
                            "You attack {} with your {weapon_name} and deal {} damage with {} penetration.",
                            unit.name_for_actions(),
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
                        game.log.log(message, Colors::LIGHT_GREEN);

                        Some((unit_id, damage))
                    }
                    AttackResult::Miss => {
                        game.log.log(
                            format!(
                                "You attack {} with your {weapon_name} and miss.",
                                unit.name_for_actions(),
                            ),
                            Colors::YELLOW,
                        );
                        None
                    }
                }
            } else {
                // TODO: attack terrains and items
                game.log.log(
                    format!("You swing in the air with your {weapon_name}.",),
                    Colors::WARM_IVORY,
                );
                None
            };

            drop(world);
            if let Some((unit_id, damage)) = hit_result {
                game.world.borrow_mut().apply_damage(unit_id, damage);
            }

            game.modes.pop();
        }
        None
    }
}
