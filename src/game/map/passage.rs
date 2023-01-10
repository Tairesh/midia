use serde::Deserialize;

// TODO: why?
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Passage {
    Passable(f32), // ticks to pass (for 2-legged human)
    Impassable,
}
