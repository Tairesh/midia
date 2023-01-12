use geometry::Direction;

use super::super::{
    super::{
        log::{LogCategory, LogEvent},
        map::{
            item::{ItemInteract, ItemView},
            terrain::{TerrainInteract, TerrainView},
        },
        Avatar, World,
    },
    Action, ActionImpl,
    ActionPossibility::{self, No, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Drop {
    pub item_id: usize,
    pub dir: Direction,
}

impl ActionImpl for Drop {
    fn is_possible(&self, actor: &Avatar, world: &World) -> ActionPossibility {
        if actor.wield.is_empty() {
            return No("You have nothing to drop".to_string());
        }
        let pos = actor.pos + self.dir;
        let mut map = world.map();
        let tile = map.get_tile(pos);
        if !tile.terrain.can_stock_items() {
            return No(format!("You can't put items on {}", tile.terrain.name()));
        }

        if let Some(item) = actor.wield.get(self.item_id) {
            let k = if matches!(self.dir, Direction::Here) {
                1.0
            } else {
                1.5
            };
            Yes((item.drop_time(actor) * k).round() as u32)
        } else {
            No("Item doesn't exists".to_string())
        }
    }

    fn on_finish(&self, action: &Action, world: &mut World) {
        let owner = action.owner_mut(world);
        let item = owner.wield.remove(self.item_id);
        let owner = action.owner(world);
        let pos = owner.pos + self.dir;
        let name = item.name();
        world.map().get_tile_mut(pos).items.push(item);
        world.log().push(LogEvent::new(
            format!("{} dropped the {name}", owner.name_for_actions()),
            pos,
            LogCategory::Info,
        ));
    }
}
