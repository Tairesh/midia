#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Passage {
    // ticks to pass (for 2-legged gazan without buffs)
    Passable(u32),
    Impassable,
    TemporaryImpassable(usize),
}

impl Passage {
    /// Units can't actually pass through other units, so this is just a dummy "big" value for pathfinding purposes.
    pub const UNIT_PASSAGE_COST: u32 = 100;

    pub fn is_passable(self) -> bool {
        self != Passage::Impassable
    }
}
