use crate::domain::{Game, Roll};

use super::GameError;

pub fn move_player_with_roll(
    game: &mut Game,
    player_name: String,
    roll: Roll,
) -> Result<(), GameError> {
    match game.get_player(player_name).as_mut() {
        Some(player) => {
            let new_location = player.location().add_roll(roll);
            player.set_location(new_location);
            game.update_player(player.clone());
            Ok(())
        }
        None => Err(GameError::PlayerNotFound),
    }
}

pub fn move_player(game: &mut Game, player_name: String) -> Result<(), GameError> {
    move_player_with_roll(game, player_name, Roll::random())
}
