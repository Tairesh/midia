use std::rc::Rc;

use tetra::Context;

pub use self::{
    alert::Alert, button::Button, fonts::Fonts, images::Images, prepared_font::PreparedFont,
    tileset::Tileset,
};

mod alert;
mod button;
mod fonts;
mod images;
mod prepared_font;
mod tileset;

// Can't put this to OnceCell because tetra::Font and tetra::Texture uses Rc<> inside
pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub button: Rc<Button>,
    pub alert: Rc<Alert>,
    pub tileset: Rc<Tileset>,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            fonts: Fonts::load(ctx)?,
            images: Images::load(ctx)?,
            button: Rc::new(Button::load(ctx)?),
            alert: Rc::new(Alert::load(ctx)?),
            tileset: Rc::new(Tileset::load(ctx)?),
        })
    }
}
