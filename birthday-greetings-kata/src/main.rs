use birthday_greetings_kata::{
    integrations::{EmailGreetingsFormatter, FlatfileEmployees, LocalCalendar},
    use_cases::greet_employees,
};

fn main() {
    let calendar = Box::new(LocalCalendar);
    let greetings_formatter = Box::new(EmailGreetingsFormatter::new());
    let employees = Box::new(FlatfileEmployees::new(
        "tests/assets/employees.txt".to_string(),
    ));

    let greetings = greet_employees(calendar, greetings_formatter, employees);

    greetings.iter().for_each(|s| println!("{}", s));
}
