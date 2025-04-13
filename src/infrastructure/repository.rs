use crate::domain::bank_account::{BankAccount, Transaction};
use crate::domain::port::BankAccountPort;
use sqlx::{query, Pool, Postgres};
use std::error::Error;

pub struct BankAccountAdapter {
    pool: Pool<Postgres>,
}

/*struct TransactionManager<'a> {
    transaction: DbTransaction<'a, Postgres>,
    pool: Pool<Postgres>,
}

impl<'a> TransactionManager<'a> {
    pub async fn transaction(&mut self) {
        let transaction = self.pool.begin().await.unwrap();
        TRANSACTION
            .scope(RefCell::new(transaction), async move {})
            .await;
    }
}

task_local! {
    static TRANSACTION: RefCell<DbTransaction<Postgres>>;
}
*/
impl BankAccountAdapter {
    async fn get_bank_account_id(&self, account_number: &str) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as("SELECT id FROM bank_account WHERE account_number = $1")
            .bind(account_number)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }
}

impl BankAccountAdapter {
    pub fn new(pool: Pool<Postgres>) -> BankAccountAdapter {
        Self { pool }
    }
}

impl BankAccountPort for BankAccountAdapter {
    async fn save_account(&self, bank_account: &BankAccount) -> Result<i32, Box<dyn Error>> {
        let mut transaction = self.pool.begin().await?;
        let result = query!(
            r#"INSERT INTO bank_account (account_number, initial_amount) VALUES ($1, $2) RETURNING ID"#,
            bank_account.account_number(),
            bank_account.initial_amount()).fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;
        Ok(result.id)
    }

    async fn save_transaction(
        &self,
        account_number: &str,
        transaction: &Transaction,
    ) -> Result<i32, Box<dyn Error>> {
        let id = self.get_bank_account_id(account_number).await?;
        let result = query!(
            r#"INSERT INTO transaction (bank_account_id, type, amount, date) VALUES ($1, $2, $3, $4) RETURNING ID"#,
            id,
            transaction.transaction_type(),
            transaction.amount(),
            transaction.date()
        ).fetch_one(&self.pool)
            .await?;
        Ok(result.id)
    }

    async fn load(&self, account_number: &str) -> Result<BankAccount, Box<dyn Error>> {
        let bank_account_row = sqlx::query!(
            "SELECT account_number, initial_amount FROM bank_account WHERE account_number = $1",
            account_number
        )
        .fetch_one(&self.pool)
        .await?;

        let transactions_rows = sqlx::query!(
        "SELECT type as transaction_type, amount, date FROM transaction WHERE bank_account_id = (SELECT id FROM bank_account WHERE account_number = $1)",
        account_number
    )
            .fetch_all(&self.pool)
            .await?;

        let transactions: Vec<Transaction> = transactions_rows
            .into_iter()
            .map(|row| match row.transaction_type.as_str() {
                "deposit" => Transaction::Deposit {
                    date: row.date,
                    amount: row.amount,
                },
                "withdraw" => Transaction::Withdraw {
                    date: row.date,
                    amount: row.amount,
                },
                _ => panic!("Unknown transaction type"),
            })
            .collect();

        let account = BankAccount::create_from_existing_account(
            bank_account_row.account_number,
            transactions,
            bank_account_row.initial_amount,
        );
        Ok(account)
    }
}
