use once_cell::sync::OnceCell;

use crate::assets::Names;

static INSTANCE: OnceCell<GameData> = OnceCell::new();

#[derive(Debug)]
pub struct GameData {
    pub names: Names,
}

impl GameData {
    pub fn load() -> Self {
        Self {
            names: Names::load(),
        }
    }

    pub fn instance() -> &'static Self {
        INSTANCE.get_or_init(Self::load)
    }
}

#[cfg(test)]
mod tests {
    use super::GameData;

    #[test]
    fn data_load() {
        let data = GameData::load();
        assert!(data.names.male_names.len() > 0);
        assert!(data.names.female_names.len() > 0);
        assert!(data.names.names.len() > 0);
    }
}
