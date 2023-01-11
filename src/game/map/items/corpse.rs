use geometry::Point;

use super::super::{
    super::{bodies::Body, races::Personality},
    item::{ItemInteract, ItemView},
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Corpse {
    pub character: Personality,
    pub body: Body,
}

impl Corpse {
    pub fn new(character: Personality, body: Body) -> Self {
        Self { character, body }
    }
}

impl ItemView for Corpse {
    fn name(&self) -> String {
        let mut adjectives = Vec::new();
        if self.body.wear.is_empty() {
            adjectives.push("naked");
        }
        let age_name = if let Some(bp) = self.body.parts.get(&Point::new(0, 0)) {
            adjectives.push(bp.freshness().adjective());
            bp.age_name()
        } else {
            "dismembered".to_string()
        };
        format!("{} {age_name} corpse", adjectives.join(" "))
    }

    fn looks_like(&self) -> &'static str {
        "corpse"
    }
}

impl ItemInteract for Corpse {
    fn mass(&self) -> u32 {
        // TODO: return bodies mass
        60_000
    }
}
