#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Serialize};
use rocket::{State, tokio};
use rusqlite::{params, Connection};
use web3::transports::Http;
use web3::types::{TransactionRequest, U256};
use web3::Web3;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Deserialize;
use uuid::Uuid;
use std::env;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    id: String,
    private_key: String,
    key_phrase: Option<String>,
    balance: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    from: String,
    to: String,
    amount: f64,
    status: String,
    tx_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WalletRequest {
    private_key: String,
    key_phrase: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TransactionRequestDTO {
    to: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    response: bool,
    message: String,
    data: Option<T>,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_wallet, get_balance, send_token, get_transaction])
        .manage(init_db())
}

// Initialize the SQLite database (for storing wallet info)
fn init_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallets (
            id TEXT PRIMARY KEY,
            private_key TEXT NOT NULL,
            key_phrase TEXT,
            balance REAL NOT NULL
        )",
        [],
    ).unwrap();
    conn
}

// Utility function to generate a new wallet (a private key & key phrase)
fn generate_wallet() -> Wallet {
    let id = Uuid::new_v4().to_string();
    let private_key = "somePrivateKey123"; // This should be generated using an actual crypto library
    let key_phrase = Some("keyphrase_example".to_string()); // This could be generated too

    Wallet {
        id,
        private_key: private_key.to_string(),
        key_phrase,
        balance: 0.0,
    }
}

// Middleware to validate if the user has a valid token (JWT)
fn validate_token(token: &str) -> bool {
    // Example validation - a proper JWT validation should be done here
    token == "valid_token" // This is just a placeholder for illustration
}

// API route to create a wallet
#[post("/wallet/create", data = "<wallet_req>")]
async fn create_wallet(wallet_req: Json<WalletRequest>, state: &State<Connection>) -> Json<ApiResponse<Wallet>> {
    if !validate_token("some_jwt_token") {
        return Json(ApiResponse {
            response: false,
            message: "Invalid token".to_string(),
            data: None,
        });
    }

    // Simulating the wallet generation
    let wallet = generate_wallet();

    let mut stmt = state.inner().prepare("INSERT INTO wallets (id, private_key, key_phrase, balance) VALUES (?, ?, ?, ?)").unwrap();
    stmt.execute(params![wallet.id, wallet.private_key, wallet.key_phrase, wallet.balance]).unwrap();

    Json(ApiResponse {
        response: true,
        message: "Wallet created successfully".to_string(),
        data: Some(wallet),
    })
}

// API route to get balance of a wallet
#[get("/wallet/balance/<id>")]
async fn get_balance(id: String, state: &State<Connection>) -> Json<ApiResponse<f64>> {
    let mut stmt = state.inner().prepare("SELECT balance FROM wallets WHERE id = ?").unwrap();
    let balance = stmt.query_row(params![id], |row| row.get(0)).unwrap_or(0.0);

    Json(ApiResponse {
        response: true,
        message: "Balance fetched successfully".to_string(),
        data: Some(balance),
    })
}

// API route to send tokens from one wallet to another
#[post("/wallet/send/<from_id>", data = "<transaction_req>")]
async fn send_token(from_id: String, transaction_req: Json<TransactionRequestDTO>, state: &State<Connection>) -> Json<ApiResponse<Transaction>> {
    let to = &transaction_req.to;
    let amount = transaction_req.amount;

    // Here we would connect to a blockchain and send the transaction
    // For example: Web3::send_transaction()
    let tx_hash = "some_tx_hash".to_string(); // Placeholder for actual transaction hash

    let mut stmt = state.inner().prepare("SELECT balance FROM wallets WHERE id = ?").unwrap();
    let from_balance: f64 = stmt.query_row(params![from_id], |row| row.get(0)).unwrap_or(0.0);

    if from_balance < amount {
        return Json(ApiResponse {
            response: false,
            message: "Insufficient balance".to_string(),
            data: None,
        });
    }

    // Update the sender's balance (deduct the sent amount)
    let new_from_balance = from_balance - amount;
    let mut stmt = state.inner().prepare("UPDATE wallets SET balance = ? WHERE id = ?").unwrap();
    stmt.execute(params![new_from_balance, from_id]).unwrap();

    // Update the recipient's balance (add the received amount)
    let mut stmt = state.inner().prepare("SELECT balance FROM wallets WHERE id = ?").unwrap();
    let mut rows = stmt.query(params![to]).unwrap();
    let mut recipient_balance = 0.0;
    if let Some(row) = rows.next().unwrap() {
        recipient_balance = row.get(0).unwrap();
    }
    let new_recipient_balance = recipient_balance + amount;
    let mut stmt = state.inner().prepare("UPDATE wallets SET balance = ? WHERE id = ?").unwrap();
    stmt.execute(params![new_recipient_balance, to]).unwrap();

    let transaction = Transaction {
        from: from_id,
        to: to.clone(),
        amount,
        status: "Success".to_string(),
        tx_hash: Some(tx_hash),
    };

    Json(ApiResponse {
        response: true,
        message: "Transaction successful".to_string(),
        data: Some(transaction),
    })
}

// API route to fetch transaction details
#[get("/transaction/<tx_hash>")]
async fn get_transaction(tx_hash: String, state: &State<Connection>) -> Json<ApiResponse<Transaction>> {
    // Simulate fetching the transaction from the blockchain or database
    let transaction = Transaction {
        from: "sender_id".to_string(),
        to: "recipient_id".to_string(),
        amount: 10.0,
        status: "Completed".to_string(),
        tx_hash: Some(tx_hash),
    };

    Json(ApiResponse {
        response: true,
        message: "Transaction details fetched successfully".to_string(),
        data: Some(transaction),
    })
}
