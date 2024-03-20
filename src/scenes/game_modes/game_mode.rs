use enum_dispatch::enum_dispatch;
use tetra::Context;

use crate::game::World;
use crate::scenes::game_modes::Cursor;
use crate::scenes::{implements::GameScene, Transition};

use super::implements::{
    Interacting, MeleeAttack, Observing, PikeAttack, Shooting, Throwing, Walking,
};

#[enum_dispatch]
pub trait GameModeImpl {
    fn cursors(&self, _world: &World) -> Option<Vec<Cursor>> {
        None
    }
    fn can_push(&self, _world: &World) -> Result<(), String> {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context, _game: &mut GameScene) {}
    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> Option<Transition>;
}

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Observing,
    MeleeAttack,
    PikeAttack,
    Throwing,
    Shooting,
    Interacting,
}
