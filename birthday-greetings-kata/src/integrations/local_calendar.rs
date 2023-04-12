use chrono::Datelike;

use crate::domain::{Calendar, Date, Day, Month, Year};

pub struct LocalCalendar;

impl Calendar for LocalCalendar {
    fn today(&self) -> Date {
        let chrono_date = chrono::Local::now();
        let year = chrono_date.year();
        let month = chrono_date.month();
        let day = chrono_date.day();
        Date {
            year: Year::new(year.try_into().unwrap()),
            month: Month::new(month.try_into().unwrap()),
            day: Day::new(day.try_into().unwrap()),
        }
    }
}
