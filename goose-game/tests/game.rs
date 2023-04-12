use goose_game::{
    domain::{Game, Player},
    use_cases::add_player,
};

#[test]
fn it_adds_a_player_to_the_game() {
    let game = Game::new();

    let _result = add_player(game, Player::new("Pippo".to_string()));

    assert_eq!(game.players().len(), 1);
}
