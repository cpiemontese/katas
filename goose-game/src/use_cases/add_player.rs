use crate::domain::{Game, Player};

impl Game {
    pub fn add_player(&mut self, _player: Player) -> Game {
        *self
    }
}
