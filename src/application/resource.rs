use crate::application::error::ProblemDetail;
use crate::domain::bank_account::Transaction;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateBankAccount {
    pub initial_amount: i32,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub amount: i32,
}

#[derive(Serialize)]
pub struct BankAccountResponse {
    account_number: String,
    initial_amount: i32,
    transactions: Vec<TransactionResponse>,
    balance: i32,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction_type: String,
    pub amount: i32,
    pub date: DateTime<Utc>,
}

pub async fn create_account(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<CreateBankAccount>,
) -> Result<(), String> {
    state
        .bank_account_adapter
        .create_bank_account(account_number, payload.initial_amount)
        .await;
    Ok(())
}

pub async fn deposit(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<TransactionRequest>,
) -> Result<(), ProblemDetail> {
    state
        .bank_account_adapter
        .deposit_into_bank_account(account_number, payload.amount)
        .await;
    Ok(())
}

pub async fn withdraw(
    State(state): State<AppState>,
    Path(account_number): Path<String>,
    Json(payload): Json<TransactionRequest>,
) -> Result<(), ProblemDetail> {
    state
        .bank_account_adapter
        .with_draw_into_bank_account(account_number, payload.amount)
        .await;
    Ok(())
}

pub async fn get_account(
    State(AppState {
        bank_account_adapter,
    }): State<AppState>,
    Path(account_number): Path<String>,
) -> Result<Json<BankAccountResponse>, ProblemDetail> {
    // Récupérer les informations du compte bancaire et les transactions de la base de données
    // Retourner la réponse
    let bank_account = bank_account_adapter.get_bank_account(&account_number).await;
    if let Some(bank_account) = bank_account {
        let mut transactions = vec![];
        for t in bank_account.transactions() {
            let new_transaction = match t {
                Transaction::Deposit { .. } => TransactionResponse {
                    transaction_type: "Deposit".to_string(),
                    amount: *t.amount(),
                    date: *t.date(),
                },
                Transaction::Withdraw { .. } => TransactionResponse {
                    transaction_type: "WithDraw".to_string(),
                    amount: *t.amount(),
                    date: *t.date(),
                },
            };
            transactions.push(new_transaction);
        }
        let response = BankAccountResponse {
            account_number: bank_account.account_number().to_string(),
            initial_amount: bank_account.initial_amount(),
            transactions,
            balance: bank_account.balance(),
        };
        return Ok(Json(response));
    }
    let string = format!("account '{}' does not exist", &account_number);
    Err(ProblemDetail::new(StatusCode::NOT_FOUND, string))
}
