mod game;
mod player;

pub use game::*;
pub use player::Player;

#[derive(Clone, Copy, Debug)]
pub enum Die {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl std::ops::Add<Die> for Die {
    type Output = u8;

    fn add(self, rhs: Die) -> u8 {
        (self as u8) + (rhs as u8)
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

    pub(crate) fn add_roll(self, roll: Roll) -> Self {
        let mut new_location = self.0 + roll.total();
        if new_location > 63 {
            let cells_to_retrocede_by = new_location - 63;
            new_location -= cells_to_retrocede_by;
        }
        Location(new_location)
    }
}
