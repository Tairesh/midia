use serde::Deserialize;

// TODO: why?
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Passage {
    // ticks to pass (for 2-legged racesperson without buffs)
    Passable(f32),
    Impassable,
}
