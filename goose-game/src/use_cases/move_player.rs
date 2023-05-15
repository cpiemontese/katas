use crate::domain::{DiceRoller, Game, Location};

use super::GameError;

pub fn move_player(
    game: &mut Game,
    dice_roller: &dyn DiceRoller,
    player_name: String,
) -> Result<(), GameError> {
    match game.get_player(player_name).as_mut() {
        Some(player) => {
            let roll = dice_roller.roll();
            let mut new_location = player.location().add_roll(&roll);

            if new_location.is_bridge_location() {
                new_location = Location::after_landing_on_bridge();
            } else {
                while new_location.is_a_goose_location() {
                    new_location = new_location.add_roll(&roll);
                }

                if new_location.has_exceeded_winning_location() {
                    new_location = new_location.bounce();
                }
            }

            player.set_location(new_location);
            game.update_player(player.clone());

            Ok(())
        }
        None => Err(GameError::PlayerNotFound),
    }
}
