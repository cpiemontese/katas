use goose_game::{
    domain::{Game, Player},
    use_cases::add_player::add_player_to_game,
};

#[test]
fn it_adds_two_players_to_the_game() {
    let player_pippo = Player::new("Pippo".to_string());
    let player_pluto = Player::new("Pluto".to_string());

    let mut game = Game::new();

    let result = add_player_to_game(&mut game, player_pippo)
        .and_then(|()| add_player_to_game(&mut game, player_pluto));

    assert!(result.is_ok());

    let players = game.players();

    assert!(players.contains(&Player::new("Pippo".to_string())));
    assert!(players.contains(&Player::new("Pluto".to_string())));

    assert_eq!(players.len(), 2);
}

#[test]
fn it_returns_error_when_adding_duplicate_player() {
    let player_pippo = Player::new("Pippo".to_string());

    let mut game = Game::new();

    let result = add_player_to_game(&mut game, player_pippo.clone());
    assert!(result.is_ok());

    let result = add_player_to_game(&mut game, player_pippo);
    assert!(result.is_err());

    assert_eq!(game.players().len(), 1);
}
