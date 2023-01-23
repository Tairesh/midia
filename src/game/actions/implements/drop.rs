use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{TerrainInteract, TerrainView},
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Drop {
    pub dir: Direction,
}

impl ActionImpl for Drop {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.wield.active_hand().is_none() {
            return No("You have nothing to drop".to_string());
        }
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.can_stock_items() {
            return No(format!(
                "You can't put items on the {}",
                tile.terrain.name()
            ));
        }

        if let Some(item) = actor.wield.active_hand() {
            let k = if matches!(self.dir, Direction::Here) {
                1.0
            } else {
                1.5
            };
            Yes((item.drop_time() * k).round() as u32)
        } else {
            No("[DEBUG] Item doesn't exists".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let owner = action.owner_mut(world);
        let item = owner.wield.take_from_active_hand().unwrap();
        let owner = action.owner(world);
        let pos = owner.pos + self.dir;
        let name = item.name().to_string();
        world.map().get_tile_mut(pos).items.push(item);
        world.log().push(LogEvent::new(
            format!("{} dropped the {name}", owner.name_for_actions()),
            pos,
            LogCategory::Info,
        ));
    }
}

#[cfg(test)]
mod tests {
    use geometry::{Direction, Point};

    use crate::game::map::items::helpers::STONE_AXE;
    use crate::game::map::terrains::Dirt;
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Item};

    use super::Drop;

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.map().get_tile_mut(Point::new(0, 0)).terrain = Dirt::default().into();
        world.map().get_tile_mut(Point::new(0, 0)).items.clear();
        world.player_mut().wield.clear();
        world.player_mut().wield.wield(Item::new(STONE_AXE));

        world.player_mut().action = Some(
            Action::new(
                0,
                Drop {
                    dir: Direction::Here,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();

        assert_eq!(Point::new(0, 0), world.player().pos);
        assert!(world.player().wield.is_empty());
        let mut map = world.map();
        assert_eq!(1, map.get_tile(Point::new(0, 0)).items.len());
        let item = map.get_tile(Point::new(0, 0)).items.first().unwrap();
        assert_eq!(item.proto().id, STONE_AXE);
    }
}
