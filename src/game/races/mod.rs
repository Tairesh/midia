pub use body::BodySlot;
pub use fur_color::FurColor;
pub use gender::Gender;
pub use main_hand::MainHand;
pub use personality::{Appearance, Mind, Personality};
pub use race::{PlayableRace, Race};
pub use sex::Sex;

mod body;
mod fur_color;
mod gender;
mod main_hand;
mod personality;
mod race;
mod sex;

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as personality;
}
