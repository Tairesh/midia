use std::collections::HashMap;

use serde::Deserialize;

use crate::game::races::{Race, Sex};

#[derive(Deserialize, Debug)]
pub struct NamesPack {
    pub names: HashMap<Race, HashMap<Sex, Vec<String>>>,
}
