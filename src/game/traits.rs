use crate::assets::Sprite;

pub trait Name {
    fn name(&self) -> &str;
}

pub trait LooksLike {
    fn looks_like(&self) -> Sprite;
}
