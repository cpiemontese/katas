#[derive(Clone, Copy, Debug)]
pub struct Year(u16);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Month(u8);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Day(u8);

impl Year {
    pub fn new(year: u16) -> Year {
        Year(year)
    }

    pub fn is_leap(&self) -> bool {
        self.0 % 4 == 0 && (self.0 % 100 != 0 || self.0 % 400 == 0)
    }
}

impl Month {
    pub fn new(month: u8) -> Month {
        Month(month)
    }
}

impl Day {
    pub fn new(day: u8) -> Day {
        Day(day)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Date {
    pub year: Year,
    pub month: Month,
    pub day: Day,
}

impl Date {
    pub fn new(year: Year, month: Month, day: Day) -> Self {
        Date { year, month, day }
    }

    pub fn day(&self) -> Day {
        self.day
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn from_ymd_str(ymd: &str, sep: char) -> Self {
        let split: Vec<&str> = ymd.split(sep).collect();
        let year: u16 = split[0].parse().expect("Should be able to parse year");
        let month: u8 = split[1].parse().expect("Should be able to parse month");
        let day: u8 = split[2].parse().expect("Should be able to parse day");

        Date {
            year: Year::new(year),
            month: Month::new(month),
            day: Day::new(day),
        }
    }
}
