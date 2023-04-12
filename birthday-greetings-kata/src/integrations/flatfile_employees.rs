use std::fs;

use crate::domain::{Date, Employee, Employees, FirstName};

pub struct FlatfileEmployees {
    employees_file_path: String,
}

impl FlatfileEmployees {
    pub fn new(path: String) -> Self {
        FlatfileEmployees {
            employees_file_path: path,
        }
    }
}

impl Employees for FlatfileEmployees {
    fn all(&self) -> Vec<crate::domain::Employee> {
        let file_as_string = fs::read_to_string(self.employees_file_path.as_str())
            .expect("Should be able to read flatfile from path");

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
}
