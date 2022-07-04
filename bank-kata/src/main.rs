use bank_kata::{account::Account, transaction_handler::TransactionHandlerImpl};

/// Problem description
///
/// Create a simple bank application with the following features:
/// - deposit into account
/// - withdraw from account
/// - print a bank statement to the console
///
/// **Acceptance criteria**
///
/// Statement should have transactons in the following format:
///
/// ```
/// > DATE       | AMOUNT  | BALANCE
/// > 10/04/2014 | 500.00  | 1400.00
/// > 02/04/2014 | -100.00 | 900.00
/// > 01/04/2014 | 1000.00 | 1000.00
/// ```
///
/// Contraints
///
/// 1. Start with the following structure
///
/// ```
/// struct Account;
///
/// impl Account {
///     pub fn deposit(&self, amount: i32) -> () {
///         todo!()
///     }
///
///     pub fn withdraw(&self, amount: i32) -> () {
///         todo!()
///     }
///
///     pub fn print_statement(&self) -> () {
///         todo!()
///     }
/// }
/// ```
///
/// 2. You are not allowed to add any other public method to this (except for `new`)
/// 3. Use Strings and Integers for date and amounts (KISS)
/// 4. Don't worry about spacing in the printed statement (optionally format)

fn main() {
    let transaction_handler = TransactionHandlerImpl::new();
    let mut account = Account::new(transaction_handler);
    account.deposit(1000);
    account.withdraw(100);
    account.deposit(500);
    account.print_statement();
}
