use crate::domain::{Game, Player};

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Tried to add duplicate player.")]
    TriedToAddDuplicatePlayer,
}

pub fn add_player_to_game(game: &mut Game, player: Player) -> Result<(), GameError> {
    if game.add_player(player) {
        return Ok(());
    } else {
        return Err(GameError::TriedToAddDuplicatePlayer);
    }
}
