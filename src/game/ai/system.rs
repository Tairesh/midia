use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::game::{Action, World};

use super::{BasicMonsterAI, DummyAI};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AI {
    Dummy,
    BasicMonster,
}

pub trait AIImpl {
    fn plan(&mut self, unit_id: usize, world: &World) -> Option<Action>;
}

static AI_MANAGER: OnceCell<Mutex<AIManager>> = OnceCell::new();

#[derive(Debug)]
pub struct AIManager {
    dummy: DummyAI,
    basic_monster: BasicMonsterAI,
}

impl AIManager {
    pub fn instance() -> MutexGuard<'static, AIManager> {
        AI_MANAGER
            .get_or_init(|| {
                Mutex::new(AIManager {
                    dummy: DummyAI {},
                    basic_monster: BasicMonsterAI {},
                })
            })
            .lock()
            .expect("AI_MANAGER: ALL YOUR AI ARE BELONGS TO US")
    }

    pub fn plan(&mut self, ai: AI, unit_id: usize, world: &World) -> Option<Action> {
        match ai {
            AI::Dummy => self.dummy.plan(unit_id, world),
            AI::BasicMonster => self.basic_monster.plan(unit_id, world),
        }
    }
}
