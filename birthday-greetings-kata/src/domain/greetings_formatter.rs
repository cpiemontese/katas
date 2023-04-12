use super::Employee;

pub trait GreetingsFormatter {
    fn format(&self, employee: Employee) -> String;
}
