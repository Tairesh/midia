use roguemetry::Direction;

use super::super::{
    super::traits::Name,
    super::{
        log::{LogCategory, LogEvent},
        map::{TerrainInteract, TerrainInteractAction, TerrainView},
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct DropMainHand {
    pub dir: Direction,
}

impl ActionImpl for DropMainHand {
    fn is_possible(&self, actor_id: usize, world: &World) -> ActionPossibility {
        let actor = world.units.get_unit(actor_id);
        let inventory = actor.inventory();
        if inventory.is_none() || inventory.unwrap().main_hand().is_none() {
            return No("You have nothing to drop".to_string());
        }
        let pos = actor.pos() + self.dir;
        let Some(tile) = world.map.get_tile_opt(pos) else {
            return No("There is no space to drop items there".to_string());
        };
        if !tile.terrain.supports_action(TerrainInteractAction::Drop) {
            return No(format!(
                "You can't put items on the {}",
                tile.terrain.name()
            ));
        }

        let item = inventory.unwrap().main_hand().unwrap();
        let k = if matches!(self.dir, Direction::Here) {
            1.0
        } else {
            1.5
        };
        Yes((item.drop_time() * k).round() as u32)
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let item = action
            .owner_mut(world)
            .inventory_mut()
            .unwrap()
            .main_hand_take()
            .unwrap();
        let pos = action.owner(world).pos() + self.dir;
        let name = item.name().to_string();
        world.map.get_tile_mut(pos).items.push(item);
        world.log.push(LogEvent::new(
            format!(
                "{} dropped the {name}",
                action.owner(world).name_for_actions()
            ),
            pos,
            LogCategory::Info,
        ));
    }
}

#[cfg(test)]
mod tests {
    use roguemetry::{Direction, Point};

    use crate::game::map::items::helpers::GOD_AXE;
    use crate::game::map::terrains::Dirt;
    use crate::game::world::tests::prepare_world;
    use crate::game::{Action, Avatar, Item};

    use super::DropMainHand;

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.map.get_tile_mut(Point::new(0, 0)).terrain = Dirt::default().into();
        world.map.get_tile_mut(Point::new(0, 0)).items.clear();
        let player = world.units.player_mut();
        player.inventory_mut().unwrap().clear();
        player.inventory_mut().unwrap().wield(Item::new(GOD_AXE));

        let action = Action::new(
            0,
            DropMainHand {
                dir: Direction::Here,
            }
            .into(),
            &world,
        )
        .unwrap();
        world.units.player_mut().set_action(Some(action));
        world.tick();

        assert_eq!(Point::new(0, 0), world.units.player().pos());
        assert!(world
            .units
            .player()
            .inventory()
            .unwrap()
            .main_hand()
            .is_none());
        assert_eq!(1, world.map.get_tile(Point::new(0, 0)).items.len());
        let item = world.map.get_tile(Point::new(0, 0)).items.first().unwrap();
        assert_eq!(item.proto().id, GOD_AXE);
    }
}
