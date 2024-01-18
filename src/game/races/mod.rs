pub use body::BodySlot;
pub use body_color::{next_color, BodyColor};
pub use gender::Gender;
pub use main_hand::MainHand;
pub use personality::{Appearance, Mind, Personality};
pub use race::{PlayableRace, Race};
pub use sex::Sex;

mod body;
mod body_color;
mod gender;
mod main_hand;
mod personality;
mod race;
mod sex;

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as personality;
}
