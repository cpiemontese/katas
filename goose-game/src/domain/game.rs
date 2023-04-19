use std::collections::HashMap;

use crate::domain::Player;

#[derive(Clone)]
pub struct Game {
    players: HashMap<String, Player>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
        }
    }

    pub(crate) fn add_player(&mut self, player: Player) -> bool {
        if self.players.contains_key(&player.name()) {
            return false;
        }
        self.players.insert(player.name(), player);
        true
    }

    pub fn players(&self) -> Vec<Player> {
        self.players
            .clone()
            .into_iter()
            .map(|(_name, player)| player)
            .collect()
    }
}
