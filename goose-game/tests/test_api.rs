use goose_game::domain::{DiceRoller, Game, Location, Player, Roll};

pub struct RiggedDiceRoller {
    roll: Roll,
}

impl RiggedDiceRoller {
    pub fn new(roll: Roll) -> Self {
        RiggedDiceRoller { roll }
    }
}

impl DiceRoller for RiggedDiceRoller {
    fn roll(&self) -> Roll {
        self.roll.clone()
    }
}

pub fn is_number_of_players_expected(game: &Game, expected_len: usize) -> bool {
    game.players().len() == expected_len
}

pub fn is_player_at_expected_location(player: &Player, expected_location: &Location) -> bool {
    player.location() == *expected_location
}

pub fn find_player(game: &Game, player_name: String) -> Option<Player> {
    game.players()
        .iter()
        .find(|p| p.name() == player_name)
        .cloned()
}
