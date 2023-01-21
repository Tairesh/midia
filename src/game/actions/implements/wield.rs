use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        Avatar, World,
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
            match world.player().wield.can_wield(item.is_two_handed()) {
                Ok(_) => Yes(item.wield_time().round() as u32),
                Err(e) => No(e),
            }
        } else {
            No("There is nothing to pick up".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let pos = action.owner(world).pos + self.dir;
        let item = world.map().get_tile_mut(pos).items.pop();
        if let Some(item) = item {
            let name = item.name().to_string();
            action.owner_mut(world).wield.wield(item);
            world.log().push(LogEvent::new(
                format!(
                    "{} wield{} the {}",
                    action.owner(world).name_for_actions(),
                    if action.owner(world).is_player() {
                        ""
                    } else {
                        "s"
                    },
                    name
                ),
                pos,
                LogCategory::Success,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::{axe, random_book, shovel};
    use crate::game::world::tests::prepare_world;
    use crate::game::Action;

    use super::WieldFromGround;

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world.map().get_tile_mut(Point::new(1, 0)).items.push(axe());

        assert!(world.player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        world.player_mut().action = Some(
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

        let item = world.player().wield.active_hand().unwrap();
        assert_eq!(item.proto.id, axe().proto.id);
        assert_eq!(0, world.map().get_tile(Point::new(1, 0)).items.len());
    }

    #[test]
    fn test_wielding_two_handed_items() {
        let mut world = prepare_world();
        world.player_mut().wield.wield(shovel());
        world.player_mut().wield.swap_items();
        assert!(world.player().wield.can_wield(false).is_err());

        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world.map().get_tile_mut(Point::new(1, 0)).items.push(axe());
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
        world.player_mut().wield.wield(random_book());
        assert!(world.player().wield.can_wield(false).is_err());
        world.player_mut().wield.swap_items();
        assert!(world.player().wield.can_wield(true).is_err());
        assert!(world.player().wield.can_wield(false).is_ok());

        world.map().get_tile_mut(Point::new(1, 0)).items.clear();
        world
            .map()
            .get_tile_mut(Point::new(1, 0))
            .items
            .push(random_book());

        world.player_mut().action = Some(
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

        let item = world.player().wield.active_hand().unwrap();
        assert_eq!(item.proto.id, random_book().proto.id);
        assert_eq!(0, world.map().get_tile(Point::new(1, 0)).items.len());
        assert!(world.player().wield.can_wield(false).is_err());
    }
}
