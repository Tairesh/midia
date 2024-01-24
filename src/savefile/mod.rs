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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::game::world::tests::prepare_world;
    use crate::game::{Dice, SkillLevel};

    use super::{delete, load, load_world, SAVEFILES_FOLDER};

    const SAVEFILE_NAME: &str = "test.save";

    #[test]
    fn test_save_and_load() {
        let path = [SAVEFILES_FOLDER, SAVEFILE_NAME]
            .iter()
            .collect::<PathBuf>();
        let mut world = prepare_world();
        world.meta.path = path.clone();
        world.units.player_mut().personality.mind.name = "test".to_string();
        world
            .units
            .player_mut()
            .personality
            .char_sheet
            .attributes
            .agility = Dice::D12;
        world
            .units
            .player_mut()
            .personality
            .char_sheet
            .skills
            .shooting = SkillLevel::D8;
        world.save();

        let meta = load(&path).unwrap();
        assert_eq!(meta.name, world.meta.name);
        assert_eq!(meta.current_tick, world.meta.current_tick);
        assert_eq!(meta.version, world.meta.version);
        assert_eq!(meta.seed, world.meta.seed);

        let world2 = load_world(&path).unwrap();
        assert_eq!(world.game_view.zoom, world2.game_view.zoom);
        assert_eq!(world.units.player().pos, world2.units.player().pos);
        assert_eq!("test", world2.units.player().personality.mind.name);
        assert_eq!(
            Dice::D12,
            world2
                .units
                .player()
                .personality
                .char_sheet
                .attributes
                .agility
        );
        assert_eq!(
            SkillLevel::D8,
            world2.units.player().personality.char_sheet.skills.shooting
        );

        delete(&path);
    }
}
