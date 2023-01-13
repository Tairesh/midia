mod app;
mod assets;
mod colors;
mod game;
mod input;
mod savefile;
mod scenes;
mod settings;
mod ui;
mod window;

pub const NAME: &str = "Followers Of The Midia";
pub const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    env!("MIDIA_VERSION_POSTFIX")
);

fn main() -> tetra::Result {
    window::create_window(format!("{NAME} {VERSION}"))?.run(app::App::new)
}
