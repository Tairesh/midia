use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        traits::Name,
        Attribute, Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct WieldFromGround {
    pub dir: Direction,
}

impl ActionImpl for WieldFromGround {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.personality.char_sheet.shock {
            return No("You are in shock".to_string());
        }

        let pos = actor.pos + self.dir;
        if let Some(item) = world.map().get_tile(pos).items.last() {
            match actor.wield.can_wield(item.is_two_handed()) {
                Ok(..) => Yes(item.wield_time().round() as u32),
                Err(e) => No(e),
            }
        } else {
            No("There is nothing to pick up".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let mut units = world.units_mut();
        let owner = action.owner_mut(&mut units);
        let pos = owner.pos + self.dir;
        let item = world.map().get_tile_mut(pos).items.pop();
        if let Some(item) = item {
            let mut msg = format!(
                "{} wield{} the {}.",
                owner.name_for_actions(),
                if owner.is_player() { "" } else { "s" },
                item.name()
            );
            if owner.is_player() {
                if let Some(dice) = item.melee_damage().minimum_strength {
                    if owner
                        .personality
                        .char_sheet
                        .get_attribute_with_modifiers(Attribute::Strength)
                        .dice()
                        < dice
                    {
                        msg += " You are not strong enough to use it effectively.";
                    }
                }
            }
            owner.wield.wield(item);
            world
                .log()
                .push(LogEvent::new(msg, pos, LogCategory::Success));
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::{random_book, GOD_AXE, STONE_SHOVEL};
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Item};

    use super::WieldFromGround;

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(Item::new(GOD_AXE));

        assert!(world.units().player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        world.units_mut().player_mut().action = Some(
            Action::new(
                0,
                WieldFromGround {
                    dir: Direction::East,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();

        let units = world.units();
        let item = units.player().wield.main_hand().unwrap();
        assert_eq!(item.proto().id, GOD_AXE);
        assert_eq!(0, world.map().get_tile(Point::new(1, 0)).items.len());
    }

    #[test]
    fn test_wielding_two_handed_items() {
        let world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(STONE_SHOVEL));
        assert!(world.units().player().wield.can_wield(true).is_err());
        assert!(world.units().player().wield.can_wield(false).is_err());

        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(Item::new(GOD_AXE));
        assert!(Action::new(
            0,
            WieldFromGround {
                dir: Direction::East,
            }
            .into(),
            &world,
        )
        .is_err());
    }

    #[test]
    fn test_wielding_one_handed_items() {
        let mut world = prepare_world();
        world
            .units_mut()
            .player_mut()
            .wield
            .wield(Item::new(GOD_AXE));
        assert!(world.units().player().wield.can_wield(false).is_ok());
        assert!(world.units().player().wield.can_wield(true).is_err());

        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(random_book());

        world.units_mut().player_mut().action = Some(
            Action::new(
                0,
                WieldFromGround {
                    dir: Direction::East,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();

        let units = world.units();
        let item = units.player().wield.main_hand().unwrap();
        assert_eq!(item.proto().id, random_book().proto().id);
        assert_eq!(0, world.map().get_tile(Point::new(1, 0)).items.len());
        assert!(world.units().player().wield.can_wield(false).is_err());
    }
}
