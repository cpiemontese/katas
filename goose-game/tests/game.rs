use goose_game::domain::{Game, Player};

#[test]
fn it_adds_two_players_to_the_game() {
    let player_pippo = Player::new("Pippo".to_string());
    let player_pluto = Player::new("Pluto".to_string());

    let game = Game::new()
        .add_player(player_pippo)
        .add_player(player_pluto);

    let players = game.players();

    assert!(players.contains(&Player::new("Pippo".to_string())));
    assert!(players.contains(&Player::new("Pluto".to_string())));

    assert_eq!(game.players().len(), 2);
}
