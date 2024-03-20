use crate::game::{TerrainInteract, TerrainInteractAction, Tile, World};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerCommand {
    Open,
    Close,
    Read,
    Drop,
    WieldFromGround,
    Examine,
}

impl PlayerCommand {
    pub fn highlight_tile(self, tile: &Tile) -> bool {
        match self {
            Self::Examine => false,
            Self::WieldFromGround => !tile.items.is_empty(),
            _ => tile.terrain.supports_action(self.into()),
        }
    }

    pub fn can_start(self, world: &World) -> Result<(), String> {
        match self {
            Self::Drop => {
                if world.units().player().inventory.main_hand().is_none() {
                    return Err("You have nothing to drop!".to_string());
                }
            }
            Self::WieldFromGround => {
                return world.units().player().inventory.can_wield_any();
            }
            _ => {}
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
            PlayerCommand::Examine => TerrainInteractAction::Examine,
            PlayerCommand::WieldFromGround => TerrainInteractAction::WieldFromGround,
        }
    }
}
