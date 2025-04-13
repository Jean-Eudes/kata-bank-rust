use std::error::Error;
use crate::domain::bank_account::{BankAccount, Transaction};

pub trait BankAccountPort {
    async fn save_account(&self, bank_account: &BankAccount) -> Result<i32, Box<dyn Error>>;
    async fn save_transaction<'a,>(
        &'a self,
        bank_account: &str,
        transaction: &'a Transaction,
    ) -> Result<i32, Box<dyn Error>>;
    async fn load(&self, account_number: &str) -> Result<BankAccount, Box<dyn Error>>;
}
