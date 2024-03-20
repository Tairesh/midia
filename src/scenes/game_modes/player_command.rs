use crate::game::{TerrainInteract, TerrainInteractAction, Tile, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerCommand {
    Open,
    Close,
    Read,
    Drop,
}

impl PlayerCommand {
    pub fn highlight_tile(self, tile: &Tile) -> bool {
        tile.terrain.supports_action(self.into())
    }

    pub fn can_start(self, world: &World) -> Result<(), String> {
        if self == Self::Drop && world.units().player().inventory.main_hand().is_none() {
            return Err("You have nothing to drop!".to_string());
        }

        Ok(())
    }
}

impl From<PlayerCommand> for TerrainInteractAction {
    fn from(command: PlayerCommand) -> Self {
        match command {
            PlayerCommand::Open => TerrainInteractAction::Open,
            PlayerCommand::Close => TerrainInteractAction::Close,
            PlayerCommand::Read => TerrainInteractAction::Read,
            PlayerCommand::Drop => TerrainInteractAction::Drop,
        }
    }
}
