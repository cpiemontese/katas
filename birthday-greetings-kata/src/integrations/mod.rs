use std::fs;

use crate::domain::{Date, Employee, FirstName};

mod email_greetings_formatter;
mod flatfile_employees;
mod local_calendar;

pub use email_greetings_formatter::EmailGreetingsFormatter;
pub use flatfile_employees::FlatfileEmployees;
pub use local_calendar::LocalCalendar;

pub fn read_employees_from_flatfile(path: &str) -> Vec<Employee> {
    let file_as_string =
        fs::read_to_string(path).expect("Should be able to read flatfile from path");

    file_as_string
        .split('\n')
        .into_iter()
        .skip(1)
        .filter_map(|employee_line| {
            let employee_parts = employee_line
                .split(',')
                .into_iter()
                .map(|s| s.trim())
                .collect::<Vec<&str>>();

            if employee_parts.len() != 4 {
                return None;
            }

            let first_name = FirstName::new(employee_parts[1].to_string());
            let birthday = Date::from_ymd_str(employee_parts[2], '/');
            Some(Employee::new(first_name, birthday))
        })
        .collect()
}
