pub struct Account;

impl Account {
    pub fn new() -> Self {
        Account {}
    }

    pub fn deposit(&self, amount: i32) -> () {
        todo!()
    }

    pub fn withdraw(&self, amount: i32) -> () {
        todo!()
    }

    pub fn print_statement(&self) -> () {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_deposit() {
        let account = Account::new();
        account.deposit(10);
    }
}
