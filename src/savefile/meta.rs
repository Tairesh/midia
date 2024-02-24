use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::VERSION;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meta {
    #[serde(skip)]
    pub path: PathBuf,
    pub name: String,
    pub seed: u64,
    pub version: String,
    pub time: SystemTime,
    pub current_tick: u128,
}

impl Meta {
    pub fn new(name: impl Into<String>, seed: u64) -> Self {
        Self {
            path: PathBuf::default(),
            name: name.into(),
            version: VERSION.to_string(),
            time: SystemTime::now(),
            current_tick: 0,
            seed,
        }
    }

    pub fn with_path(mut self, path: &Path) -> Self {
        self.path = path.into();
        self
    }

    pub fn update_before_save(&mut self) {
        self.time = SystemTime::now();
        self.version = VERSION.to_string();
    }
}

impl Eq for Meta {}

impl PartialEq<Self> for Meta {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PartialOrd<Self> for Meta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Meta {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}
