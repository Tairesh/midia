use super::super::{
    super::{Avatar, World},
    ActionImpl,
    ActionPossibility::{self, Yes},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub struct Skip {}

impl ActionImpl for Skip {
    fn is_possible(&self, _actor: &Avatar, _world: &World) -> ActionPossibility {
        Yes(1)
    }
}
