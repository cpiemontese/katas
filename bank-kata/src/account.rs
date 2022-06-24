use std::fmt;

use crate::transaction_handler::TransactionHandler;

pub struct Account<T>
where
    T: TransactionHandler,
{
    transaction_handler: T,
}

impl<T> Account<T>
where
    T: TransactionHandler,
{
    pub fn new(transaction_handler: T) -> Self {
        Account {
            transaction_handler,
        }
    }

    pub fn deposit(&mut self, amount: i32) {
        self.transaction_handler.deposit(amount);
    }

    pub fn withdraw(&mut self, amount: i32) {
        self.transaction_handler.withdraw(amount);
    }

    pub fn print_statement(&self) {
        let history = self.transaction_handler.history();
        println!("DATE | AMOUNT | TOTAL");
        for (i, h) in history.iter().enumerate() {
            if i == history.len() - 1 {
                print!("{} | {}{} | {}", h)?
            } else {
                println!("{} | {}{} | {}", h)?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::transaction_handler::MockTransactionHandler;

    use super::Account;

    #[test]
    fn can_deposit() {
        let mock = MockTransactionHandler::new()
            .expect_deposit()
            .with(eq(10))
            .times(1);

        let account = Account::new(mock);

        account.deposit(10);
    }
}
