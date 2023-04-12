use crate::domain::Player;

#[derive(Clone, Copy)]
pub struct Game;

#[derive(thiserror::Error, Debug, Clone)]
pub enum GameError {
    #[error("Tried to add duplicate player.")]
    TriedToAddDuplicatePlayer(String),
}

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn players(&self) -> Vec<Player> {
        vec![
            Player::new("Pippo".to_string()),
            Player::new("Pluto".to_string()),
        ]
    }
}
