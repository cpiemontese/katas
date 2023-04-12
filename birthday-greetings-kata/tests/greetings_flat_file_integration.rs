use birthday_greetings_kata::{integrations, use_cases};

use crate::test_api::{TestCalendar, TestEmployees, TestGreetingsFormatter};

mod test_api;

#[test]
fn it_greets_employees_with_data_from_flat_file() {
    let employees_from_flatfile =
        integrations::read_employees_from_flatfile("tests/assets/employees.txt");
    let employees = TestEmployees::new(employees_from_flatfile);
    let calendar = TestCalendar::new(test_api::date_from_iso_str("2002-10-08"));

    let greeting_message = use_cases::greet_employees(
        Box::new(calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(greeting_message, vec!["Happy birthday, dear John!"]);
}
