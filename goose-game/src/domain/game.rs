use crate::domain::Player;

#[derive(Clone, Copy)]
pub struct Game;

impl Game {
    pub fn new() -> Self {
        Game {}
    }

    pub fn players(&self) -> Vec<Player> {
        vec![Player::new("Pippo".to_string())]
    }
}
