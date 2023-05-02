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

impl std::ops::Add<Die> for Die {
    type Output = u8;

    fn add(self, rhs: Die) -> u8 {
        (self as u8) + (rhs as u8)
    }
}
