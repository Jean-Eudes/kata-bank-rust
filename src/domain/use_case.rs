use tracing::info;
use crate::domain::bank_account::{BankAccount, BankAccountPort};

pub struct BankAccountUseCase<T> where T : BankAccountPort {
    adapter: T,
}

impl<T> BankAccountUseCase<T> where T : BankAccountPort
{
    pub fn new(adapter: T) -> BankAccountUseCase<T> {
        BankAccountUseCase { adapter }
    }

    pub async fn create_bank_account(
        &self,
        account_number: String,
        initial_amount: i32,
    ) {
        info!(message = "creation d'un compte", account_number = account_number, amount = initial_amount);

        let account = BankAccount::create_new_account(account_number, initial_amount);
        let _ = self.adapter.save_account(&account).await;
    }

    pub async fn deposit_into_bank_account(
        &self,
        account_number: String,
        amount: i32,
    ) {
        info!(message = "depot sur le compte", account_number = account_number, amount = amount);

        let mut account = self.adapter.load(account_number).await.unwrap();
        account.deposit(amount);
        let option = account.last();
        let _ = self.adapter.save_transaction(&account, option.unwrap()).await;
    }

    pub async fn with_draw_into_bank_account(
        &self,
        account_number: String,
        amount: i32,
    ) {
        info!(message = "retrait sur le compte", account_number = account_number, amount = amount);

        let mut account = self.adapter.load(account_number).await.unwrap();
        account.with_draw(amount);
        let option = account.last();
        let _ = self.adapter.save_transaction(&account, option.unwrap()).await;
    }

    pub async fn get_bank_account(
        &self,
        account_number: String,
    ) -> Option<BankAccount> {
        self.adapter.load(account_number).await.ok()
    }
}
