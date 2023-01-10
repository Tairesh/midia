pub use gender::Gender;
pub use hair_color::HairColor;
pub use main_hand::MainHand;
pub use personality::{age_name, Appearance, Mind, Personality};
pub use skin_tone::SkinTone;

mod gender;
mod hair_color;
pub mod helpers;
mod main_hand;
mod personality;
mod skin_tone;

#[cfg(test)]
pub mod tests {
    pub use super::personality::tests as personality;
}
