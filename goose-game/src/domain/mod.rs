mod game;
mod player;

pub use game::*;
pub use player::Player;

#[derive(Clone, Debug)]
pub struct Die(u8);

impl Die {
    pub fn new(value: u8) -> Result<Self, String> {
        if !(1..6).contains(&value) {
            return Err(format!("Cannot create 6-sided die with value {}", value));
        }
        Ok(Die(value))
    }
}

impl std::ops::Add<Die> for Die {
    type Output = u8;

    fn add(self, rhs: Die) -> u8 {
        self.0 + rhs.0
    }
}

#[derive(Clone, Debug)]
pub struct Roll {
    die1: Die,
    die2: Die,
}

impl Roll {
    pub fn new(die1: Die, die2: Die) -> Self {
        Roll { die1, die2 }
    }

    pub(crate) fn total(&self) -> u8 {
        self.die1.0 + self.die2.0
    }
}

#[derive(Clone, Debug, PartialEq)]
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

    pub fn add_roll(mut self, roll: Roll) -> Self {
        let mut new_location = self.0 + roll.total();
        if new_location > 63 {
            let cells_to_retrocede_by = new_location - 63;
            new_location -= cells_to_retrocede_by;
        }
        self.0 = new_location;
        self
    }
}
