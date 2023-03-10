use tetra::{graphics::Texture, Context};

#[derive(Debug)]
pub struct Images {
    pub bg: Texture,
    pub logo: Texture,
}

impl Images {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            bg: Texture::from_encoded(ctx, include_bytes!("../../assets/img/bg.jpg"))?,
            logo: Texture::from_encoded(ctx, include_bytes!("../../assets/img/logo.png"))?,
        })
    }
}
