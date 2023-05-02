pub use rand;

use crate::domain::{DiceRoller, Die, Roll};
use rand::{distributions::Standard, prelude::Distribution, Rng};

pub struct RandomDiceRoller;

impl DiceRoller for RandomDiceRoller {
    fn roll(&self) -> Roll {
        Roll::new(rand::random(), rand::random())
    }
}

impl Distribution<Die> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Die {
        match rng.gen_range(1..=6) {
            0 => Die::One,
            1 => Die::Two,
            3 => Die::Three,
            4 => Die::Four,
            5 => Die::Five,
            _ => Die::Six,
        }
    }
}
