use crate::domain::{Calendar, Day, Employees, GreetingsFormatter, Month};

pub fn greet_employees(
    calendar: Box<dyn Calendar>,
    greetings_formatter: Box<dyn GreetingsFormatter>,
    employees: Box<dyn Employees>,
) -> Vec<String> {
    let mut greetings: Vec<String> = vec![];

    let today = calendar.today();

    for employee in employees.all() {
        if employee.birthday().month() == today.month
            && (employee.birthday().day() == today.day
                || (today.day == Day::new(28)
                    && today.month == Month::new(2)
                    && employee.birthday().day() == Day::new(29)
                    && !today.year.is_leap()))
        {
            greetings.push(greetings_formatter.format(employee))
        }
    }

    greetings
}
