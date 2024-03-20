pub use action::Action;
pub use action_type::{ActionImpl, ActionType};
pub use attack_target::AttackTarget;

mod action;
mod action_type;
mod attack_target;
pub mod implements;

#[derive(Debug)]
pub enum ActionPossibility {
    Yes(u32),
    // length in ticks
    No(String), // reason why not
}
