use tetra::{
    graphics::text::{Font, Text},
    Context,
};

#[derive(Debug, Clone)]
pub struct PreparedFont {
    pub font: Font,
    pub line_height: f32,
}

impl PreparedFont {
    pub fn new(ctx: &mut Context, font: Font) -> Self {
        Self {
            line_height: detect_font_height(ctx, font.clone()),
            font,
        }
    }
}

/// Find the tallest symbol in the ASCII range
fn detect_font_height(ctx: &mut Context, font: Font) -> f32 {
    let all_chars = (32..=126) // ASCII range for printable characters
        .map(|i| i as u8 as char)
        .collect::<String>();
    let bounds = Text::new(all_chars, font).get_bounds(ctx).unwrap();

    bounds.height
}
