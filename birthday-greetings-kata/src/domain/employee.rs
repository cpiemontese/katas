use super::Date;

#[derive(Clone, Debug)]
pub struct FirstName(String);

#[derive(Clone, Debug)]
pub struct Employee {
    birthday: Date,
    first_name: FirstName,
}

pub trait Employees {
    fn all(&self) -> Vec<Employee>;
}

impl FirstName {
    pub fn new(first_name: String) -> Self {
        FirstName(first_name)
    }
}

impl Employee {
    pub fn new(first_name: FirstName, birthday: Date) -> Self {
        Employee {
            first_name,
            birthday,
        }
    }

    pub fn birthday(&self) -> Date {
        self.birthday
    }
}

impl std::fmt::Display for FirstName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.first_name)
    }
}
