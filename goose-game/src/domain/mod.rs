mod die;
mod game;
mod player;

pub use die::*;
pub use game::*;
pub use player::Player;

const WINNING_CELL: u8 = 63;
const BRIDGE_LOCATION: u8 = 6;
const AFTER_BRIDGE_LOCATION: u8 = 12;
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

    pub fn starting_location() -> Self {
        Location(0)
    }

    pub fn after_landing_on_bridge() -> Self {
        Location(AFTER_BRIDGE_LOCATION)
    }

    pub fn is_winning_location(&self) -> bool {
        self.0 == WINNING_CELL
    }

    pub fn is_bridge_location(&self) -> bool {
        self.0 == BRIDGE_LOCATION
    }

    pub fn has_exceeded_winning_location(&self) -> bool {
        self.0 > WINNING_CELL
    }

    pub fn is_a_goose_location(&self) -> bool {
        GOOSE_LOCATIONS.contains(&self.0)
    }

    pub fn bounce(&self) -> Self {
        let cells_to_retrocede_by = self.0 - WINNING_CELL;
        Location(WINNING_CELL - cells_to_retrocede_by)
    }

    pub fn add_roll(&self, roll: &Roll) -> Self {
        Location(self.0 + roll.total())
    }
}
