use std::collections::HashMap;

use crate::domain::{Location, Player};

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

    pub(crate) fn get_player(&self, player_name: String) -> Option<Player> {
        self.players.get(&player_name).map(|p| p.to_owned())
    }

    pub(crate) fn update_player(&mut self, player: Player) {
        self.players.insert(player.name(), player);
    }

    pub fn is_winner(&self, player_name: String) -> bool {
        match self.get_player(player_name) {
            Some(player) => player.location() == Location::end(),
            None => false,
        }
    }

    pub fn players(&self) -> Vec<Player> {
        self.players
            .clone()
            .into_iter()
            .map(|(_name, player)| player)
            .collect()
    }
}
