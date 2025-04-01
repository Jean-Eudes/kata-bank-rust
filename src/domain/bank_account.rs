use crate::domain::bank_account::Transaction::{Deposit, Withdraw};
use chrono::{DateTime, Utc};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct BankAccount {
    account_number: String,
    transactions: Vec<Transaction>,
    initial_amount: i32,
}

pub enum Transaction {
    Deposit { date: DateTime<Utc>, amount: i32 },
    Withdraw { date: DateTime<Utc>, amount: i32 },
}

impl BankAccount {
    pub fn create_new_account(account_number: String, initial_amount: i32) -> Self {
        BankAccount {
            account_number,
            transactions: Vec::new(),
            initial_amount,
        }
    }
    pub fn create_from_existing_account(
        account_number: String,
        transactions: Vec<Transaction>,
        initial_amount: i32,
    ) -> Self {
        BankAccount {
            account_number,
            transactions,
            initial_amount,
        }
    }

    pub fn deposit(&mut self, amount: i32) {
        let transaction = Deposit {
            date: Utc::now(),
            amount,
        };
        self.transactions.push(transaction);
    }

    pub fn with_draw(&mut self, amount: i32) {
        self.transactions.push(Withdraw {
            date: Utc::now(),
            amount,
        });
    }

    pub fn balance(&self) -> i32 {
        let sum: i32 = self
            .transactions
            .iter()
            .map(|t| match t {
                Deposit {
                    date: _date,
                    amount,
                } => *amount,
                Withdraw {
                    date: _date,
                    amount,
                } => -amount,
            })
            .sum();
        self.initial_amount + sum
    }

    pub fn last(&self) -> Option<&Transaction> {
        self.transactions.last()
    }

    pub fn account_number(&self) -> &str {
        &self.account_number
    }

    pub fn initial_amount(&self) -> i32 {
        self.initial_amount
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

impl Transaction {
    pub fn transaction_type(&self) -> &str {
        match self {
            Deposit { .. } => "deposit",
            Withdraw { .. } => "withdraw",
        }
    }

    pub fn amount(&self) -> &i32 {
        match self {
            Deposit { amount, .. } | Withdraw { amount, .. } => amount,
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        match self {
            Deposit { date, .. } | Withdraw { date, .. } => date,
        }
    }
}

impl Display for BankAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BankAccount {{\n  account_number: {},\n  initial_amount: {},\n  transactions: [\n",
            self.account_number, self.initial_amount
        )?;
        for transaction in &self.transactions {
            writeln!(f, "    {},", transaction)?;
        }
        write!(f, "  ]\n}}")
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Transaction::Deposit { date, amount } => {
                write!(f, "Deposit {{ date: {}, amount: {} }}", date, amount)
            }
            Transaction::Withdraw { date, amount } => {
                write!(f, "Withdraw {{ date: {}, amount: {} }}", date, amount)
            }
        }
    }
}

pub trait BankAccountPort {
    async fn save_account(&self, bank_account: &BankAccount) -> Result<i32, Box<dyn Error>>;
    async fn save_transaction(
        &self,
        bank_account: &BankAccount,
        transaction: &Transaction,
    ) -> Result<i32, Box<dyn Error>>;
    async fn load(&self, account_number: String) -> Result<BankAccount, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn should_create_new_bank_account() {
        // Given / When
        let account = BankAccount::create_new_account("account_number".to_string(), 100);

        // Then
        assert_eq!(account.account_number, "account_number");
        assert_eq!(account.transactions.len(), 0);
        assert_eq!(account.initial_amount, 100);
    }

    #[test]
    fn should_deposit_to_bank_account() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        account.deposit(1000);

        // Then
        assert_eq!(
            matches!(
                account.transactions[0],
                Deposit {
                    date: _date,
                    amount: 1000
                }
            ),
            true
        );
    }

    #[test]
    fn should_with_draw_to_bank_account() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 100);

        // When
        account.with_draw(500);

        // Then
        assert_eq!(
            matches!(
                account.transactions[0],
                Withdraw {
                    date: _date,
                    amount: 500
                }
            ),
            true
        );
    }
    #[test]
    fn should_compute_balance() {
        // Given
        let mut account = BankAccount::create_new_account("account_number".to_string(), 1000);

        // When
        account.with_draw(500);
        account.deposit(2000);

        // Then
        assert_eq!(account.balance(), 2_500);
    }

    #[derive(Debug)]
    struct WithCell {
        a: u32,
        b: Cell<u32>,
    }
    #[test]
    fn should_test_cell() {
        // Given
        let account = BankAccount::create_new_account("account_number".to_string(), 1000);
        let mut account = Cell::new(account);
        let a = WithCell {
            a: 12,
            b: Cell::new(10),
        };

        println!("{a:?}");
        a.b.set(200);
        println!("{a:?}");
        

        // When
        account.get_mut().with_draw(500);
        account.get_mut().deposit(2000);

        // Then
        assert_eq!(account.get_mut().balance(), 2_500);
    }
}
