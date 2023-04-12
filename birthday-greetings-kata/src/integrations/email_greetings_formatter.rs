use crate::domain::{Employee, GreetingsFormatter};

pub struct EmailGreetingsFormatter;

impl EmailGreetingsFormatter {
    pub fn new() -> Self {
        EmailGreetingsFormatter {}
    }
}

impl GreetingsFormatter for EmailGreetingsFormatter {
    fn format(&self, employee: Employee) -> String {
        format!(
            "Subject: Happy birthday!\n\nHappy birthday, dear {}!",
            employee
        )
    }
}
