use tetra::{
    graphics::{NineSlice, Rectangle, Texture},
    Context,
};

#[derive(Debug)]
pub struct Button {
    pub texture: Texture,
    pub default: NineSlice,
    pub disabled: NineSlice,
    pub pressed: NineSlice,
    pub hovered: NineSlice,
}

impl Button {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            texture: Texture::from_encoded(ctx, include_bytes!("../../assets/img/button.png"))?,
            default: NineSlice::new(Rectangle::new(0.0, 0.0, 46.0, 14.0), 3.0, 3.0, 3.0, 4.0),
            hovered: NineSlice::new(Rectangle::new(0.0, 14.0, 46.0, 14.0), 3.0, 3.0, 3.0, 4.0),
            pressed: NineSlice::new(Rectangle::new(0.0, 28.0, 46.0, 14.0), 3.0, 3.0, 4.0, 3.0),
            disabled: NineSlice::new(Rectangle::new(0.0, 42.0, 46.0, 14.0), 3.0, 3.0, 3.0, 4.0),
        })
    }
}
