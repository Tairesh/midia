pub use action::Action;
pub use action_impl::ActionImpl;
pub use action_type::ActionType;

mod action;
mod action_impl;
mod action_type;
pub mod implements;

pub enum ActionPossibility {
    Yes(u32),   // length in ticks
    No(String), // reason why not
}
