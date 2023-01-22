#[derive(Debug)]
pub enum Passage {
    // ticks to pass (for 2-legged gazan without buffs)
    Passable(f32),
    Impassable,
}
