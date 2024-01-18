use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

// TODO: move settings to a separate directory

const PATH: &str = "settings.json";
static INSTANCE: OnceCell<Mutex<Settings>> = OnceCell::new();

trait Validate {
    fn validate(&mut self);
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
            width: 1366,
            height: 768,
            fullscreen: false,
        }
    }
}

impl Validate for WindowSettings {
    fn validate(&mut self) {
        self.width = self.width.clamp(1366, 1920);
        self.height = self.height.clamp(768, 1280);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct DebugSettings {
    pub show_fps: bool,
    // TODO: debug log, backtrace, god-mode, etc.
}

impl Validate for DebugSettings {
    fn validate(&mut self) {
        // do nothing
    }
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

impl Validate for InputSettings {
    fn validate(&mut self) {
        self.repeat_interval = self.repeat_interval.clamp(1, 1000);
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Settings {
    pub window: WindowSettings,
    pub debug: DebugSettings,
    pub input: InputSettings,
}

impl Settings {
    pub fn instance() -> MutexGuard<'static, Settings> {
        INSTANCE
            .get_or_init(|| Mutex::new(load(PATH)))
            .lock()
            .expect("CATS: ALL YOUR SETTINGS ARE BELONGS TO US")
    }

    pub fn save(&mut self) {
        self.validate();
        save(self, PATH);
    }

    pub fn validate(&mut self) {
        self.window.validate();
        self.debug.validate();
        self.input.validate();
    }
}

fn load_from_file(path: &'static str) -> Result<Settings, ()> {
    let path = Path::new(path);
    if !path.is_file() {
        return Err(());
    }
    let file = File::open(path).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|_| ())
        .map(|mut settings: Settings| {
            settings.validate();
            settings
        })
}

fn load(path: &'static str) -> Settings {
    load_from_file(path).unwrap_or_else(|()| {
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
    use super::Settings;

    #[test]
    fn test_invalid_settings() {
        let mut settings = Settings::default();
        settings.window.width = 1365;
        settings.window.height = 767;
        settings.input.repeat_interval = 0;
        settings.validate();

        assert_eq!(1366, settings.window.width);
        assert_eq!(768, settings.window.height);
        assert_eq!(1, settings.input.repeat_interval);
    }
}
