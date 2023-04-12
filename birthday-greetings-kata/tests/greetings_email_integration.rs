use birthday_greetings_kata::{
    domain::{Employee, FirstName},
    integrations::EmailGreetingsFormatter,
    use_cases,
};

use crate::test_api::{TestCalendar, TestEmployees};

mod test_api;

#[test]
fn it_greets_employees_with_data_from_flat_file() {
    let october_eighth_of_1982 = test_api::date_from_iso_str("1982-10-08");
    let employees = TestEmployees::new(vec![Employee::new(
        FirstName::new("John".to_string()),
        october_eighth_of_1982,
    )]);

    let calendar = TestCalendar::new(test_api::date_from_iso_str("2002-10-08"));

    let greeting_message = use_cases::greet_employees(
        Box::new(calendar),
        Box::new(EmailGreetingsFormatter::new()),
        Box::new(employees),
    );

    assert_eq!(
        greeting_message,
        vec!["Subject: Happy birthday!\n\nHappy birthday, dear John!"]
    );
}
