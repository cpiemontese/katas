use crate::domain::{Game, GameError, Player};

impl Game {
    pub fn add_player(self, _player: Player) -> Result<Game, GameError> {
        Ok(self)
    }
}
