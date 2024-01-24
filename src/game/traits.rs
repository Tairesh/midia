pub trait Name {
    fn name(&self) -> &str;
}

pub trait LooksLike {
    fn looks_like(&self) -> &str;
}
