use goose_game::{
    domain::{Die, Game, Location, Player, Roll},
    use_cases::{add_player::add_player_to_game, move_player::move_player_with_roll},
};

use crate::test_api::{find_player, is_number_of_players_expected, is_player_at_expected_location};

mod test_api;

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

    assert!(is_number_of_players_expected(&game, 2));
}

#[test]
fn it_returns_error_when_adding_duplicate_player() {
    let player_pippo = Player::new("Pippo".to_string());

    let mut game = Game::new();

    let result = add_player_to_game(&mut game, player_pippo.clone());
    assert!(result.is_ok());

    let result = add_player_to_game(&mut game, player_pippo);
    assert!(result.is_err());

    assert!(is_number_of_players_expected(&game, 1));
}

#[test]
fn it_moves_player_successfully() {
    let player_pippo: Player = Player::new("Pippo".to_string());

    let mut game = Game::new();

    let result = add_player_to_game(&mut game, player_pippo.clone());
    assert!(result.is_ok());

    let roll = Roll::new(Die::Four, Die::Two);
    let result = move_player_with_roll(&mut game, player_pippo.name(), roll);
    assert!(result.is_ok());

    let moved_player = find_player(&game, player_pippo.name()).expect("Player not added to game");
    let expected_location = Location::new(6).expect("Couldn't create location");

    assert!(is_player_at_expected_location(
        &moved_player,
        &expected_location
    ))
}

#[test]
fn it_marks_player_as_winner_when_63rd_cell_is_reached() {
    let mut player_pippo: Player = Player::new("Pippo".to_string());
    player_pippo.set_location(Location::new(60).expect("Failed to setup player location"));

    let mut game = Game::new();

    let result = add_player_to_game(&mut game, player_pippo.clone());
    assert!(result.is_ok());

    let roll = Roll::new(Die::Two, Die::One);
    let result = move_player_with_roll(&mut game, player_pippo.name(), roll);
    assert!(result.is_ok());

    assert!(game.is_winner(player_pippo.name()));
}
