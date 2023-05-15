mod die;
mod game;
mod player;

pub use die::*;
pub use game::*;
pub use player::Player;

const WINNING_CELL: u8 = 63;
const BRIDGE_LOCATION: u8 = 6;
const GOOSE_LOCATIONS: [u8; 6] = [5, 9, 14, 18, 23, 27];

pub trait DiceRoller {
    fn roll(&self) -> Roll;
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
    pub fn new(cell: u8) -> Self {
        if cell > 63 {
            let cells_to_retrocede_by = cell - WINNING_CELL;
            return Location(WINNING_CELL - cells_to_retrocede_by);
        }

        Location(cell)
    }

    pub fn start() -> Self {
        Location(0)
    }

    pub fn end() -> Self {
        Location(WINNING_CELL)
    }

    pub(crate) fn add_roll(&self, roll: Roll) -> Self {
        let mut new_location = self.0 + roll.total();

        if new_location == BRIDGE_LOCATION {
            return Location(12);
        }

        while GOOSE_LOCATIONS.contains(&new_location) {
            new_location += roll.total();
        }

        if new_location > WINNING_CELL {
            let cells_to_retrocede_by = new_location - WINNING_CELL;
            new_location = WINNING_CELL - cells_to_retrocede_by;
        }

        Location(new_location)
    }
}
