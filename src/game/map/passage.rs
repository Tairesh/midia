#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Passage {
    // ticks to pass (for 2-legged gazan without buffs)
    Passable(u32),
    Impassable,
    TemporaryImpassable(usize),
}

impl Passage {
    pub fn is_passable(self) -> bool {
        self != Passage::Impassable
    }
}
