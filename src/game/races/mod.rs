pub use fur_color::FurColor;
pub use gender::Gender;
pub use main_hand::MainHand;
pub use personality::{age_name, Appearance, Mind, Personality};
pub use race::{PlayableRace, Race};
pub use skin_tone::SkinTone;

mod fur_color;
mod gender;
pub mod helpers;
mod main_hand;
mod personality;
mod race;
mod skin_tone;

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as personality;
}
