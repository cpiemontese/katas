use std::vec;

use chrono::Utc;

pub struct TransactionHandler {
    total: i32,
    history: Vec<String>,
}

enum TransactionType {
    Deposit,
    Withdraw,
}

impl TransactionHandler {
    pub fn new() -> Self {
        TransactionHandler {
            total: 0,
            history: vec![],
        }
    }

    pub fn deposit(&mut self, amount: i32) -> &mut Self {
        self.total += amount;
        self.add_transaction(amount, TransactionType::Deposit);
        self
    }

    pub fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.total >= amount {
            self.total -= amount;
            self.add_transaction(amount, TransactionType::Withdraw);
        }
        self
    }

    fn add_transaction(&mut self, amount: i32, transaction_type: TransactionType) -> &mut Self {
        let sign = if let TransactionType::Withdraw = transaction_type {
            "-"
        } else {
            ""
        };

        self.history.push(format!(
            "{} | {}{} | {}",
            Utc::now().naive_utc().date().to_string(),
            sign,
            amount,
            self.total
        ));
        self
    }
}

impl std::fmt::Display for TransactionHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "DATE | AMOUNT | TOTAL")?;
        for (i, h) in self.history.iter().enumerate() {
            if i == self.history.len() - 1 {
                write!(f, "{}", h)?
            } else {
                writeln!(f, "{}", h)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::TransactionHandler;

    #[test]
    fn can_deposit() {
        let mut th = TransactionHandler::new();
        th.deposit(10);
        assert_eq!(th.total, 10);
    }

    #[test]
    fn can_withdraw_if_has_funds() {
        let mut th = TransactionHandler::new();
        th.deposit(10).withdraw(10);
        assert_eq!(th.total, 0);
    }

    #[test]
    fn cannot_withdraw_if_has_no_funds() {
        let mut th = TransactionHandler::new();
        th.withdraw(10);
        assert_eq!(th.total, 0);
    }

    #[test]
    fn records_history_of_transactions() {
        let mut th = TransactionHandler::new();
        th.deposit(10).withdraw(10).deposit(20);
        assert_eq!(th.total, 20);
        assert_eq!(th.history.len(), 3);
        let today = Utc::now().naive_utc().date().to_string();
        assert_eq!(
            th.history,
            vec![
                format!("{} | 10 | 10", today),
                format!("{} | -10 | 0", today),
                format!("{} | 20 | 20", today)
            ]
        );
    }

    #[test]
    fn formats_on_print() {
        let mut th = TransactionHandler::new();
        th.deposit(10).withdraw(10).deposit(20);
        assert_eq!(th.total, 20);
        assert_eq!(th.history.len(), 3);
        let today = Utc::now().naive_utc().date().to_string();

        assert_eq!(
            th.to_string(),
            format!(
                "DATE | AMOUNT | TOTAL\n{} | 10 | 10\n{} | -10 | 0\n{} | 20 | 20",
                today, today, today
            )
        );
    }
}
