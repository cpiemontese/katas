mod die;
mod game;
mod player;

pub use die::*;
pub use game::*;
pub use player::Player;

#[derive(Clone, Debug)]
pub struct Roll {
    die1: Die,
    die2: Die,
}

impl Roll {
    pub fn new(die1: Die, die2: Die) -> Self {
        Roll { die1, die2 }
    }

    pub fn random() -> Self {
        Roll {
            die1: Die::random(),
            die2: Die::random(),
        }
    }

    pub(crate) fn total(&self) -> u8 {
        (self.die1 as u8) + (self.die2 as u8)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Location(u8);

impl Location {
    pub fn new(cell: u8) -> Result<Self, String> {
        if !(0..63).contains(&cell) {
            return Err("Location must be between 0 and 63".to_string());
        }
        Ok(Location(cell))
    }

    pub fn starting_location() -> Self {
        Location(0)
    }

    pub fn end_location() -> Self {
        Location(63)
    }

    pub(crate) fn add_roll(self, roll: Roll) -> Self {
        let mut new_location = self.0 + roll.total();
        if new_location > 63 {
            let cells_to_retrocede_by = new_location - 63;
            new_location = 63 - cells_to_retrocede_by;
        }
        Location(new_location)
    }
}
