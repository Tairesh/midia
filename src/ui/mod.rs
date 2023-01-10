pub use self::game_log::GameLog;
pub use self::implements::{
    alert::Alert,
    button::Button,
    image::Image,
    inputs::TextInput,
    label::{ItemDisplay, Label},
    meshy::{HoverableMesh, JustMesh},
};
pub use self::position::{AnchorX, AnchorY, Horizontal, Position, Vertical};
pub use self::traits::{
    Colorize, Disable, Draw, Focus, Hover, Positionate, Press, Stringify, UiSprite, Update,
};

mod game_log;
mod implements;
mod position;
mod traits;

pub type SomeUISpritesMut<'a> = Option<&'a mut [Box<dyn UiSprite>]>;
pub type SomeUISprites<'a> = Option<&'a [Box<dyn UiSprite>]>;
