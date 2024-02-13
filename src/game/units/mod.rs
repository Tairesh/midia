use std::collections::{HashMap, HashSet};

pub use avatar::{Avatar, AvatarView};
pub use fighter::Fighter;
pub use inventory::Inventory;
pub use monster::Monster;
pub use personality::{Appearance, Mind, PlayerPersonality};
pub use player::Player;
pub use weapon::Weapon;
pub use wear::Wear;
pub use wield::Wield;

mod avatar;
mod fighter;
mod inventory;
mod monster;
mod personality;
mod player;
mod weapon;
mod wear;
mod wield;

pub struct Units {
    units: HashMap<usize, Box<dyn Avatar>>,
    loaded_units: HashSet<usize>,
}

impl Units {
    const BUBBLE_SQUARE_RADIUS: u32 = 128 * 128;

    pub fn new(mut units: HashMap<usize, Box<dyn Avatar>>) -> Self {
        for (&id, unit) in &mut units {
            unit.set_id(id);
        }
        Self {
            units,
            loaded_units: HashSet::from([0]),
        }
    }

    pub fn player(&self) -> &Player {
        self.get_unit(0).as_player().unwrap()
    }

    pub fn player_mut(&mut self) -> &mut Player {
        self.get_unit_mut(0).as_player_mut().unwrap()
    }

    pub fn player_as_avatar(&self) -> &dyn Avatar {
        self.get_unit(0).as_fighter().as_avatar()
    }

    pub fn get_unit(&self, unit_id: usize) -> &dyn Avatar {
        self.units
            .get(&unit_id)
            .unwrap_or_else(|| {
                panic!("Trying to get unit with id {unit_id} but there is no such unit!",)
            })
            .as_ref()
    }

    pub fn get_unit_mut(&mut self, unit_id: usize) -> &mut dyn Avatar {
        self.units
            .get_mut(&unit_id)
            .unwrap_or_else(|| {
                panic!("Trying to get mutable unit with id {unit_id} but there is no such unit!",)
            })
            .as_mut()
    }

    pub fn next_unit_id(&self) -> usize {
        self.units.keys().copied().max().unwrap_or(0) + 1
    }

    pub fn add_unit(&mut self, mut unit: Box<dyn Avatar>) -> usize {
        let id = self.next_unit_id();
        unit.set_id(id);
        self.units.insert(id, unit);
        self.load_units();

        id
    }

    pub fn load_units(&mut self) {
        self.loaded_units.clear();
        let center = self.player().as_avatar().pos();
        for (&i, unit) in &self.units {
            if unit.char_sheet().is_dead() {
                continue;
            }

            let pos = unit.pos();
            let dist = pos.square_distance(center);
            if dist <= Self::BUBBLE_SQUARE_RADIUS {
                self.loaded_units.insert(i);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &dyn Avatar)> {
        self.units.iter().map(|(i, u)| (i, u.as_ref()))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut Box<dyn Avatar>)> {
        self.units.iter_mut()
    }

    pub fn loaded_units(&self) -> impl Iterator<Item = &dyn Avatar> {
        self.loaded_units
            .iter()
            .map(|&i| self.get_unit(i).as_fighter().as_avatar())
    }

    pub fn loaded_units_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Avatar>> {
        let loaded_units = self.loaded_units.clone();
        self.iter_mut().filter_map(move |(i, u)| {
            if loaded_units.contains(i) {
                Some(u)
            } else {
                None
            }
        })
    }

    pub fn unload_unit(&mut self, unit_id: usize) {
        self.get_unit_mut(unit_id).set_action(None);
        self.loaded_units.remove(&unit_id);
    }

    pub fn remove_unit(&mut self, unit_id: usize) {
        self.units.remove(&unit_id);
        self.unload_unit(unit_id);
    }
}

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as helpers;
}
