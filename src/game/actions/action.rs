use crate::game::units::Avatar;

use super::{
    super::{
        log::{LogCategory, LogEvent},
        units::Units,
        World,
    },
    ActionImpl, ActionPossibility, ActionType,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Action {
    pub owner: usize,
    pub typ: ActionType,
    pub length: u32,
    pub finish: u128,
}

impl Action {
    pub fn new(owner: usize, typ: ActionType, world: &World) -> Result<Self, String> {
        match typ.is_possible(owner, world) {
            ActionPossibility::Yes(length) => {
                let finish = world.meta.current_tick + length as u128;
                Ok(Self {
                    owner,
                    typ,
                    length,
                    finish,
                })
            }
            ActionPossibility::No(s) => Err(s),
        }
    }

    pub fn owner<'a>(&self, units: &'a Units) -> &'a dyn Avatar {
        units.get_unit(self.owner)
    }

    pub fn owner_mut<'a>(&self, units: &'a mut Units) -> &'a mut dyn Avatar {
        units.get_unit_mut(self.owner)
    }

    fn cancel_action(&self, world: &mut World, reason: String) {
        self.owner_mut(&mut world.units_mut()).set_action(None);
        if self.owner == 0 {
            world.log().push(LogEvent::new(
                reason,
                self.owner(&world.units()).pos(),
                LogCategory::Warning,
            ));
        }
    }

    /// called every tick
    pub fn act(&self, world: &mut World) {
        if let ActionPossibility::No(reason) = self.typ.is_possible(self.owner, world) {
            self.cancel_action(world, reason);
            return;
        }
        // TODO: draw stamina

        if self.finish <= world.meta.current_tick {
            self.typ.on_finish(self, world);
        } else {
            let steps = (self.finish - world.meta.current_tick) as u32;
            if steps == self.length {
                self.typ.on_start(self, world);
            } else {
                self.typ.on_step(self, world);
            }
        }
    }
}
