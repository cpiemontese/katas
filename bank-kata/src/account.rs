use crate::transaction_handler::{TransactionHandler, TransactionHistoryElement, TransactionType};

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
            let transaction_string = self.transaction_to_string(h);
            if i == history.len() - 1 {
                print!("{}", transaction_string)
            } else {
                println!("{}", transaction_string)
            }
        }
    }

    fn transaction_to_string(&self, transaction: &TransactionHistoryElement) -> String {
        let sign = if let TransactionType::Withdraw = transaction.transaction_type {
            "-"
        } else {
            ""
        };

        format!(
            "{} | {}{} | {}",
            transaction.date.as_str(),
            sign,
            transaction.amount,
            self.transaction_handler.balance()
        )
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
