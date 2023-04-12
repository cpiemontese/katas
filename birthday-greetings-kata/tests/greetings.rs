use std::vec;

use birthday_greetings_kata::{
    domain::{Employee, FirstName},
    use_cases,
};

use crate::test_api::{TestCalendar, TestEmployees, TestGreetingsFormatter};

mod test_api;

#[test]
fn it_greets_just_one_employee_if_day_differs_but_month_is_same() {
    let calendar = TestCalendar::new(test_api::date_from_iso_str("2002-10-08"));

    let october_eighth_of_1982 = test_api::date_from_iso_str("1982-10-08");
    let october_ninth_of_1982 = test_api::date_from_iso_str("1982-10-09");
    let employees = TestEmployees::new(vec![
        Employee::new(FirstName::new("John".to_string()), october_eighth_of_1982),
        Employee::new(FirstName::new("Mark".to_string()), october_ninth_of_1982),
    ]);

    let greeting_message = use_cases::greet_employees(
        Box::new(calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(greeting_message, vec!["Happy birthday, dear John!"]);
}

#[test]
fn it_greets_more_than_one_employee_if_they_share_birth_day_and_month() {
    let calendar = TestCalendar::new(test_api::date_from_iso_str("2002-10-08"));

    let october_eighth_of_1982 = test_api::date_from_iso_str("1982-10-08");
    let october_eighth_of_1992 = test_api::date_from_iso_str("1992-10-08");
    let employees = TestEmployees::new(vec![
        Employee::new(FirstName::new("John".to_string()), october_eighth_of_1982),
        Employee::new(FirstName::new("Mark".to_string()), october_eighth_of_1992),
    ]);

    let greeting_message = use_cases::greet_employees(
        Box::new(calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(
        greeting_message,
        vec!["Happy birthday, dear John!", "Happy birthday, dear Mark!"]
    );
}

#[test]
fn it_greets_just_one_employee_if_month_differs_but_day_is_same() {
    let test_calendar = TestCalendar::new(test_api::date_from_iso_str("2002-10-08"));

    let october_eighth_of_1982 = test_api::date_from_iso_str("1982-10-08");
    let november_eighth_of_1982 = test_api::date_from_iso_str("1982-09-08");
    let employees = TestEmployees::new(vec![
        Employee::new(FirstName::new("John".to_string()), october_eighth_of_1982),
        Employee::new(FirstName::new("Mark".to_string()), november_eighth_of_1982),
    ]);

    let greeting_message = use_cases::greet_employees(
        Box::new(test_calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(greeting_message, vec!["Happy birthday, dear John!"]);
}

#[test]
fn it_greets_employee_born_on_29_if_leap_year() {
    let test_calendar = TestCalendar::new(test_api::date_from_iso_str("2000-02-29"));

    let february_29th_1992 = test_api::date_from_iso_str("1992-02-29");
    let employees = TestEmployees::new(vec![Employee::new(
        FirstName::new("John".to_string()),
        february_29th_1992,
    )]);

    let greeting_message = use_cases::greet_employees(
        Box::new(test_calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(greeting_message, vec!["Happy birthday, dear John!"]);
}

#[test]
fn it_greets_employee_born_on_29_on_28_if_not_leap_year() {
    let test_calendar = TestCalendar::new(test_api::date_from_iso_str("2001-02-28"));

    let february_29th_1992 = test_api::date_from_iso_str("1992-02-29");
    let employees = TestEmployees::new(vec![Employee::new(
        FirstName::new("John".to_string()),
        february_29th_1992,
    )]);

    let greeting_message = use_cases::greet_employees(
        Box::new(test_calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    assert_eq!(greeting_message, vec!["Happy birthday, dear John!"]);
}

#[test]
fn it_does_not_greet_employee_born_on_29_on_28_if_leap_year() {
    let test_calendar = TestCalendar::new(test_api::date_from_iso_str("2000-02-28"));

    let february_29th_1992 = test_api::date_from_iso_str("1992-02-29");
    let employees = TestEmployees::new(vec![Employee::new(
        FirstName::new("John".to_string()),
        february_29th_1992,
    )]);

    let greeting_message = use_cases::greet_employees(
        Box::new(test_calendar),
        Box::new(TestGreetingsFormatter),
        Box::new(employees),
    );

    let empty_greetings: Vec<String> = vec![];
    assert_eq!(greeting_message, empty_greetings);
}
