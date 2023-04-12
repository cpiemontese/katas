use super::Date;

pub trait Calendar {
    fn today(&self) -> Date;
}
