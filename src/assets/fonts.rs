use tetra::{graphics::text::Font, Context};

use super::PreparedFont;

#[derive(Debug)]
pub struct Fonts {
    pub default: PreparedFont,
    pub header: PreparedFont,
}

impl Fonts {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        let consolab = include_bytes!("../../assets/fonts/consolab.ttf");
        let consolab16 = Font::from_vector_file_data(ctx, consolab, 16.0)?;
        let consolab24 = Font::from_vector_file_data(ctx, consolab, 24.0)?;

        Ok(Self {
            default: PreparedFont::new(ctx, consolab16),
            header: PreparedFont::new(ctx, consolab24),
        })
    }
}
