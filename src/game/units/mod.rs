use std::collections::{HashMap, HashSet};

pub use avatar::Avatar;
pub use personality::{Appearance, Mind, Personality};
pub use wear::Wear;
pub use wield::Wield;

mod avatar;
mod personality;
mod wear;
mod wield;

pub struct Units {
    units: HashMap<usize, Avatar>,
    loaded_units: HashSet<usize>,
}

impl Units {
    const BUBBLE_SQUARE_RADIUS: u32 = 128 * 128;

    pub fn new(mut units: HashMap<usize, Avatar>) -> Self {
        for (&id, unit) in &mut units {
            unit.id = id;
        }
        Self {
            units,
            loaded_units: HashSet::from([0]),
        }
    }
    pub fn player(&self) -> &Avatar {
        self.get_unit(0)
    }

    pub fn player_mut(&mut self) -> &mut Avatar {
        self.get_unit_mut(0)
    }

    pub fn get_unit(&self, unit_id: usize) -> &Avatar {
        self.units.get(&unit_id).unwrap_or_else(|| {
            panic!("Trying to get unit with id {unit_id} but there is no such unit!",)
        })
    }

    pub fn get_unit_mut(&mut self, unit_id: usize) -> &mut Avatar {
        self.units.get_mut(&unit_id).unwrap_or_else(|| {
            panic!("Trying to get mutable unit with id {unit_id} but there is no such unit!",)
        })
    }

    pub fn next_unit_id(&self) -> usize {
        self.units.keys().copied().max().unwrap_or(0) + 1
    }

    pub fn add_unit(&mut self, mut unit: Avatar) -> usize {
        let id = self.next_unit_id();
        unit.id = id;
        self.units.insert(id, unit);
        self.load_units();

        id
    }

    pub fn load_units(&mut self) {
        self.loaded_units.clear();
        let center = self.player().pos;
        for (&i, unit) in &self.units {
            if unit.personality.char_sheet.is_dead() {
                continue;
            }

            let pos = unit.pos;
            let dist = pos.square_distance(center);
            if dist <= Self::BUBBLE_SQUARE_RADIUS {
                self.loaded_units.insert(i);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &Avatar)> {
        self.units.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut Avatar)> {
        self.units.iter_mut()
    }

    pub fn loaded_units(&self) -> impl Iterator<Item = &Avatar> {
        self.loaded_units.iter().map(|&i| self.get_unit(i))
    }

    pub fn unload_unit(&mut self, unit_id: usize) {
        self.loaded_units.remove(&unit_id);
    }
}

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as helpers;
}
