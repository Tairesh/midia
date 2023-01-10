use super::super::ActionType;

pub trait Brain {
    // TODO: Brain need to know who its owner is and check the world
    // TODO: async call plan() probably?
    fn plan(&mut self);
    fn action(&self) -> Option<ActionType>;
}
