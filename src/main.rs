use crate::application::resource::{create_account, deposit, get_account, withdraw};
use crate::domain::use_case::BankAccountUseCase;
use crate::infrastructure::repository::BankAccountAdapter;
use axum::routing::{get, post, put};
use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tokio::signal;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod application;
mod domain;
mod infrastructure;

#[derive(Clone)]
struct AppState {
    bank_account_adapter: Arc<BankAccountUseCase<BankAccountAdapter>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_file(true)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    dotenv().ok(); // This line loads the environment variables from the ".env" file.
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .min_connections(5)
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let adapter = BankAccountAdapter::new(pool);
    let use_case = BankAccountUseCase::new(adapter);

    let app = Router::new()
        .route("/{account_number}", put(create_account))
        .route("/{account_number}/deposit", post(deposit))
        .route("/{account_number}/withdraw", post(withdraw))
        .route("/{account_number}", get(get_account))
        .with_state(AppState {
            bank_account_adapter: Arc::new(use_case),
        });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    tokio::select! {_ = ctrl_c => {info!("received ctrl + C")}}
}
