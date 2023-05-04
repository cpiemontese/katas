use crate::domain::{DiceRoller, Game};

use super::GameError;

pub fn move_player(
    game: &mut Game,
    dice_roller: &dyn DiceRoller,
    player_name: String,
) -> Result<(), GameError> {
    match game.get_player(player_name).as_mut() {
        Some(player) => {
            let roll = dice_roller.roll();
            let new_location = player.location().add_roll(roll);
            player.set_location(new_location);
            game.update_player(player.clone());
            Ok(())
        }
        None => Err(GameError::PlayerNotFound),
    }
}
