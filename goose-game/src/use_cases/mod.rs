pub mod add_player;
pub mod move_player;

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Tried to add duplicate player.")]
    TriedToAddDuplicatePlayer,
    #[error("Could not find player.")]
    PlayerNotFound,
}
