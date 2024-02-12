use std::path::Path;

pub use game_view::GameView;
pub use load::{has_avatar, load, load_world, savefiles, savefiles_exists};
pub use meta::Meta;
pub use save::{create, save, Error as SaveError};

mod game_view;
mod load;
mod meta;
mod save;

const SAVEFILES_FOLDER: &str = "save";

pub fn delete(path: &Path) {
    if path.exists() {
        std::fs::remove_file(path).ok();
    }
}
