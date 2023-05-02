pub use rand;
use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone, Copy, Debug)]
pub enum Die {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl Die {
    pub fn random() -> Die {
        rand::random()
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

impl std::ops::Add<Die> for Die {
    type Output = u8;

    fn add(self, rhs: Die) -> u8 {
        (self as u8) + (rhs as u8)
    }
}
