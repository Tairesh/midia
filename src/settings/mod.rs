use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

pub use input::KeyBindingAction;

use self::{debug::DebugSettings, input::InputSettings, window::WindowSettings};

mod debug;
mod input;
mod window;

const PATH: &str = "settings.json";
static INSTANCE: OnceCell<Mutex<Settings>> = OnceCell::new();

trait Validate {
    fn validate(&mut self);
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
        self.save_to_file(PATH);
    }

    pub fn validate(&mut self) {
        self.window.validate();
        self.debug.validate();
        self.input.validate();
    }

    pub fn save_to_file(&self, path: &'static str) {
        serde_json::to_writer(&File::create(Path::new(path)).unwrap(), self).ok();
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
        settings.save_to_file(path);
        settings
    })
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
