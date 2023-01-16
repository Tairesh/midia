use geometry::{Point, DIR8};
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use crate::colors::Colors;
use crate::game::{Damage, World};
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
        } else if let Some(_point) = self.target {
            let world = game.world.borrow();
            let player = world.player();
            let (weapon_name, damage, penetration) =
                if let Some(weapon) = player.wield.active_hand() {
                    if let Some(melee_damage) = &weapon.proto.melee_damage {
                        (
                            weapon.name(),
                            melee_damage.damage.roll(&player.char_sheet),
                            melee_damage.penetration,
                        )
                    } else {
                        let default_damage = Damage::default();
                        (weapon.name(), default_damage.roll(&player.char_sheet), 0)
                    }
                } else {
                    let default_damage = Damage::default();
                    ("fists", default_damage.roll(&player.char_sheet), 0)
                };

            game.log.log(
                format!(
                    "You swing with your {weapon_name} for {damage} damage and {penetration} penetration.",
                ),
                Colors::LIGHT_CORAL,
            );

            game.modes.pop();
        }
        None
    }
}
