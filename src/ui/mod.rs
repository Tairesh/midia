#![allow(unused_imports)]

pub use self::game_log::GameLog;
pub use self::implements::{
    alert::Alert,
    button::{Button, ButtonBuilder},
    image::{Image, TilesetSprite},
    inputs::TextInput,
    label::{ItemDisplay, Label},
    meshy::{HoverableMesh, JustMesh},
};
pub use self::position::{Horizontal, Position, Vertical};
pub use self::traits::{
    Colorize, Disable, Draw, Focus, Hover, Positionable, Press, Sizeable, Stringify, UiSprite,
    Update, UpdateContext, UpdateContextState,
};

mod game_log;
mod implements;
mod position;
mod traits;

pub type SomeUISpritesMut<'a> = Option<&'a mut [Box<dyn UiSprite>]>;
pub type SomeUISprites<'a> = Option<&'a [Box<dyn UiSprite>]>;
