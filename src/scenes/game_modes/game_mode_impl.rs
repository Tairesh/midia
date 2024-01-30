use enum_dispatch::enum_dispatch;
use tetra::Context;

use crate::game::World;

use super::{
    super::{implements::GameScene, SomeTransitions},
    implements::{
        Closing, Digging, Dropping, Examining, MeleeAttack, Observing, Opening, PikeAttack,
        Reading, Shooting, Throwing, Walking, WieldingFromGround,
    },
    Cursor, GameMode,
};

#[enum_dispatch]
pub trait GameModeImpl {
    fn cursors(&self, _world: &World) -> Vec<Cursor> {
        vec![]
    }
    fn can_push(&self, _world: &World) -> Result<(), String> {
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions;
    fn draw(&mut self, _ctx: &mut Context, _game: &mut GameScene) {}
}
