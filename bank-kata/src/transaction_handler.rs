use std::vec;

use chrono::Utc;

#[cfg(test)]
use mockall::automock;

#[derive(Debug)]
pub struct TransactionHistoryElement {
    pub transaction_type: TransactionType,
    pub amount: i32,
    pub date: String,
}

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
}

#[cfg_attr(test, automock)]
pub trait TransactionHandler {
    fn balance(&self) -> i32;
    fn deposit(&mut self, amount: i32) -> &mut Self;
    fn withdraw(&mut self, amount: i32) -> &mut Self;
    fn history(&self) -> &Vec<TransactionHistoryElement>;
}

pub struct TransactionHandlerImpl {
    balance: i32,
    history: Vec<TransactionHistoryElement>,
}

impl TransactionHandler for TransactionHandlerImpl {
    fn balance(&self) -> i32 {
        self.balance
    }

    fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        self.add_transaction(amount, TransactionType::Deposit);
        self
    }

    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance >= amount {
            self.balance -= amount;
            self.add_transaction(amount, TransactionType::Withdraw);
        }
        self
    }

    fn history(&self) -> &Vec<TransactionHistoryElement> {
        &self.history
    }
}

impl TransactionHandlerImpl {
    pub fn new() -> Self {
        TransactionHandlerImpl {
            balance: 0,
            history: vec![],
        }
    }

    fn add_transaction(&mut self, amount: i32, transaction_type: TransactionType) -> &mut Self {
        self.history.push(TransactionHistoryElement {
            transaction_type,
            amount,
            date: Utc::now().naive_utc().date().to_string(),
        });
        self
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn can_deposit() {
        let mut th = TransactionHandlerImpl::new();
        th.deposit(10);
        assert_eq!(th.balance, 10);
    }

    #[test]
    fn can_withdraw_if_has_funds() {
        let mut th = TransactionHandlerImpl::new();
        th.deposit(10).withdraw(10);
        assert_eq!(th.balance, 0);
    }

    #[test]
    fn cannot_withdraw_if_has_no_funds() {
        let mut th = TransactionHandlerImpl::new();
        th.withdraw(10);
        assert_eq!(th.balance, 0);
    }

    #[test]
    fn records_history_of_transactions() {
        let mut th = TransactionHandlerImpl::new();
        th.deposit(10).withdraw(10).deposit(20);
        assert_eq!(th.balance, 20);
        assert_eq!(th.history.len(), 3);
        let today = Utc::now().naive_utc().date().to_string();

        let mut history_iter = th.history().iter();

        let first_transaction = history_iter
            .next()
            .expect("First element should be present");

        assert_eq!(first_transaction.transaction_type, TransactionType::Deposit);
        assert_eq!(first_transaction.amount, 10);
        assert_eq!(first_transaction.date, today);

        let second_transaction = history_iter
            .next()
            .expect("Second element should be present");

        assert_eq!(
            second_transaction.transaction_type,
            TransactionType::Withdraw
        );
        assert_eq!(second_transaction.amount, 10);
        assert_eq!(second_transaction.date, today);

        let third_transaction = history_iter
            .next()
            .expect("Third element should be present");

        assert_eq!(third_transaction.transaction_type, TransactionType::Deposit);
        assert_eq!(third_transaction.amount, 20);
        assert_eq!(third_transaction.date, today);
    }
}
