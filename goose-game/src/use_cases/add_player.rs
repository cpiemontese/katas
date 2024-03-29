use crate::domain::{Game, Player};

use super::GameError;

pub fn add_player_to_game(game: &mut Game, player: Player) -> Result<(), GameError> {
    if game.add_player(player) {
        return Ok(());
    }

    Err(GameError::TriedToAddDuplicatePlayer)
}
