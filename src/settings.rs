use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;
use time::UtcOffset;

const DEFAULT_PATH: &str = "./settings.json";
static INSTANCE: OnceCell<Mutex<Settings>> = OnceCell::new();

#[derive(Debug)]
pub struct TimeSettings {
    pub offset: UtcOffset,
}

impl Default for TimeSettings {
    fn default() -> Self {
        Self {
            offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WindowSettings {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct DebugSettings {
    pub show_fps: bool,
    // TODO: debug log, backtrace, god-mode, etc.
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct InputSettings {
    pub repeat_interval: u32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            repeat_interval: 125,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Settings {
    pub window: WindowSettings,
    pub debug: DebugSettings,
    pub input: InputSettings,
    #[serde(skip)]
    pub time: TimeSettings,
}

impl Settings {
    pub fn instance() -> MutexGuard<'static, Settings> {
        INSTANCE
            .get_or_init(|| Mutex::new(load(DEFAULT_PATH)))
            .lock()
            .expect("Can't lock Mutex<Settings>!")
    }

    pub fn save(&mut self) {
        // TODO: self.validate();
        save(self, DEFAULT_PATH);
    }
}

fn load_from_file(path: &'static str) -> Result<Settings, ()> {
    let path = Path::new(path);
    if !path.is_file() {
        return Err(());
    }
    let file = File::open(path).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|_| ())
}

fn load(path: &'static str) -> Settings {
    load_from_file(path).unwrap_or_else(|_| {
        let settings = Settings::default();
        save(&settings, path);
        settings
    })
}

fn save(settings: &Settings, path: &'static str) {
    serde_json::to_writer(&File::create(Path::new(path)).unwrap(), settings).ok();
}

#[cfg(test)]
mod tests {
    use super::{load, save};

    const TEST_PATH: &str = "./settings-test.json";

    #[test]
    fn test_settings_load_and_save() {
        let mut settings = load(TEST_PATH);
        settings.window.width = 987;
        save(&settings, TEST_PATH);

        let settings = load(TEST_PATH);
        assert_eq!(987, settings.window.width);

        std::fs::remove_file(TEST_PATH).ok();
    }
}
