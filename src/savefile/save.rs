use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::game::World;

use super::{Meta, SAVEFILES_FOLDER};

#[derive(Debug)]
pub enum Error {
    System(String),
    Serialize(String),
    FileExists,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialize(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::System(e.to_string())
    }
}

fn world_seed(seed: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    hasher.finish()
}

pub fn create(name: &str, seed: &str) -> Result<PathBuf, Error> {
    make_dir()?;
    let name = name.trim().replace('\n', "");
    let path = name_to_path(name.as_str());
    if path.is_file() {
        return Err(Error::FileExists);
    }
    let mut file = File::create(&path).map_err(Error::from)?;
    let meta = Meta::new(name, world_seed(seed)).with_path(&path);
    file.write_all(
        serde_json::to_string(&meta)
            .map_err(Error::from)?
            .as_bytes(),
    )
    .map_err(Into::into)
    .map(|()| path)
}

fn serialize_world(world: &World) -> Result<String, Error> {
    let mut data = serde_json::to_string(&world.meta).map_err(Error::from)?;
    data.push('\n');
    data.push_str(
        serde_json::to_string(&world.game_view)
            .map_err(Error::from)?
            .as_str(),
    );
    data.push('\n');
    data.push_str(
        serde_json::to_string(&world.log)
            .map_err(Error::from)?
            .as_str(),
    );
    for (_, unit) in world.units.iter() {
        data.push('\n');
        data.push_str(serde_json::to_string(unit).map_err(Error::from)?.as_str());
    }
    data.push_str("\n/units");
    for coords in world.map.changed.clone() {
        let chunk = world.map.chunks.get(&coords).expect("Missing chunk");
        data.push('\n');
        data.push_str(serde_json::to_string(chunk).map_err(Error::from)?.as_str());
    }
    data.push_str("\n/chunks");

    Ok(data)
}

pub fn save(world: &World) -> Result<(), Error> {
    make_dir()?;
    let mut file = File::create(&world.meta.path).map_err(Error::from)?;
    file.write_all(serialize_world(world)?.as_bytes())
        .map_err(Into::into)
}

fn make_dir() -> Result<(), Error> {
    let dir = Path::new(SAVEFILES_FOLDER);
    if !dir.exists() {
        std::fs::create_dir(dir).map_err(Error::from)?;
    }
    Ok(())
}

fn name_to_path(name: &str) -> PathBuf {
    let file_name = name.replace(['/', '\\'], "").replace(' ', "_");
    [SAVEFILES_FOLDER, (file_name + ".save").as_str()]
        .iter()
        .collect()
}
