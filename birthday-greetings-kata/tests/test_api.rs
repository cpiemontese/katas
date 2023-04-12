use birthday_greetings_kata::domain::{Calendar, Date, Employee, Employees, GreetingsFormatter};

pub struct TestGreetingsFormatter;

impl GreetingsFormatter for TestGreetingsFormatter {
    fn format(&self, employee: Employee) -> String {
        format!("Happy birthday, dear {}!", employee)
    }
}

pub fn date_from_iso_str(date_str: &str) -> Date {
    Date::from_ymd_str(date_str, '-')
}

pub struct TestCalendar {
    today: Date,
}

impl TestCalendar {
    pub fn new(today: Date) -> Self {
        TestCalendar { today }
    }
}

impl Calendar for TestCalendar {
    fn today(&self) -> Date {
        self.today
    }
}

pub struct TestEmployees {
    employees: Vec<Employee>,
}

impl TestEmployees {
    pub fn new(employees: Vec<Employee>) -> Self {
        TestEmployees { employees }
    }
}

impl Employees for TestEmployees {
    fn all(&self) -> Vec<Employee> {
        self.employees.clone()
    }
}
