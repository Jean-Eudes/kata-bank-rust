use crate::domain::bank_account::BankAccount;
use crate::domain::port::BankAccountPort;
use tracing::info;

pub struct BankAccountUseCase<T>
where
    T: BankAccountPort,
{
    adapter: T,
}

impl<T> BankAccountUseCase<T>
where
    T: BankAccountPort,
{
    pub fn new(adapter: T) -> BankAccountUseCase<T> {
        BankAccountUseCase { adapter }
    }

    pub async fn create_bank_account(&self, account_number: String, initial_amount: i32) {
        info!(
            message = "creation d'un compte",
            account_number = account_number,
            amount = initial_amount
        );

        let account = BankAccount::create_new_account(account_number, initial_amount);
        let _ = self.adapter.save_account(&account).await;
    }

    pub async fn deposit_into_bank_account(&self, account_number: String, amount: i32) {
        info!(
            message = "depot sur le compte",
            account_number = account_number,
            amount = amount
        );

        let mut account = self.adapter.load(&account_number).await.unwrap();
        let deposit = account.deposit(amount);
        let _ = self
            .adapter
            .save_transaction(&account_number, deposit)
            .await;
    }

    pub async fn with_draw_into_bank_account(&self, account_number: String, amount: i32) {
        info!(
            message = "retrait sur le compte",
            account_number = account_number,
            amount = amount
        );

        let mut account = self.adapter.load(&account_number).await.unwrap();

        let transaction = account.with_draw(amount);
        let _ = self
            .adapter
            .save_transaction(&account_number, transaction)
            .await;
    }

    pub async fn get_bank_account(&self, account_number: &str) -> Option<BankAccount> {
        self.adapter.load(account_number).await.ok()
    }
}
