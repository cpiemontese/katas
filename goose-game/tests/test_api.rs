use goose_game::{
    domain::{DiceRoller, Game, Player, Roll},
    use_cases::{move_player, GameError},
};

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

pub fn find_player(game: &Game, player_name: String) -> Option<Player> {
    game.players()
        .iter()
        .find(|p| p.name() == player_name)
        .cloned()
}

pub fn move_player_with_roll(
    game: &mut Game,
    player_name: String,
    roll: Roll,
) -> Result<(), GameError> {
    move_player::move_player(game, &RiggedDiceRoller::new(roll), player_name)
}
