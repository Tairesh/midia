use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct RollResult {
    pub natural: u8,
    pub total: i8,
}

impl RollResult {
    pub fn new(natural: u8, total: i8) -> Self {
        RollResult { natural, total }
    }

    pub fn successes(self) -> u8 {
        self.total.max(0) as u8 / 4
    }
}
