use crate::assets::Sprite;

pub trait Name {
    fn name(&self) -> String;
}

pub trait LooksLike {
    fn looks_like(&self) -> Sprite;
}
