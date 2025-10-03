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
pub use self::layout::Layout;
pub use self::position::{Horizontal, Position, Vertical};
pub use self::traits::{
    Colorize, Disable, Draw, Focus, HasLayout, HasSize, Hover, Positionable, Press, Stringify,
    UiSprite, Update, UpdateContext, UpdateContextState,
};
use tetra::Context;

mod game_log;
mod implements;
mod layout;
mod position;
mod traits;

pub type SomeUISpritesMut<'a> = Option<&'a mut [Box<dyn UiSprite>]>;
pub type SomeUISprites<'a> = Option<&'a [Box<dyn UiSprite>]>;

pub fn draw_sprites(ctx: &mut Context, sprites: &mut [Box<dyn UiSprite>]) {
    for sprite in sprites {
        if sprite.visible() {
            sprite.draw(ctx);
        }
    }
}
