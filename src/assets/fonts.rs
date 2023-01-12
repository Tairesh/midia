use tetra::{graphics::text::Font, Context};

use super::PreparedFont;

#[derive(Debug)]
pub struct Fonts {
    pub default: PreparedFont,
    pub header: PreparedFont,
    pub logo: PreparedFont,
    pub title: PreparedFont,
}

impl Fonts {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        let consolab = include_bytes!("../../assets/fonts/consolab.ttf");
        let universe = include_bytes!("../../assets/fonts/Boecklins Universe.ttf");

        let consolab16 = Font::from_vector_file_data(ctx, consolab, 16.0)?;
        let consolab24 = Font::from_vector_file_data(ctx, consolab, 24.0)?;
        let universe48 = Font::from_vector_file_data(ctx, universe, 48.0)?;
        let universe72 = Font::from_vector_file_data(ctx, universe, 72.0)?;

        Ok(Self {
            default: PreparedFont::new(ctx, consolab16),
            header: PreparedFont::new(ctx, consolab24),
            logo: PreparedFont::new(ctx, universe72),
            title: PreparedFont::new(ctx, universe48),
        })
    }
}
