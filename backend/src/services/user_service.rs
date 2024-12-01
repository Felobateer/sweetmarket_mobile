#[macro_use] extern crate rocket;

use models::{Users, Payments};
use utils::{HttpException, User_roles, Email_type, WalletService};
use rocket::{State, serde::{Serialize, json::Json}};
use rocket::tokio;
use rusqlite::{Connection, params};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize};
use rocket::data::ByteUnit;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    response: bool,
    message: String,
    data: Option<User>,
}

#[derive(Debug, Deserialize)]
struct UserSignUp {
    email: String,
    password: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![create_user, login_user, update_user])
        .manage(init_db())
}

// Initialize the database
fn init_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    ).unwrap();
    conn
}

// Utility function to hash passwords
async fn hash_password(password: &str) -> String {
    hash(password, 8).unwrap() // 8 rounds of hashing
}

// Utility function to create JWT token
fn create_jwt(user_id: i32) -> String {
    let secret = env::var("SECRET_JWT").unwrap_or("mysecret".to_string());
    let claims = jsonwebtoken::Claims {
        sub: user_id.to_string(),
        exp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600, // Expiry: 1 hour
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

// Handle user creation (sign-up)
#[post("/user/signup", data = "<user>")]
async fn create_user(user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    let mut stmt = state.inner().prepare("SELECT id FROM users WHERE email = ?").unwrap();
    let mut rows = stmt.query(params![email]).unwrap();
    
    if let Some(_) = rows.next().unwrap() {
        return Json(ApiResponse {
            response: false,
            message: "User already exists".to_string(),
            data: None,
        });
    }

    let hashed_password = hash_password(&password).await;
    let mut stmt = state.inner().prepare("INSERT INTO users (email, password) VALUES (?, ?)").unwrap();
    stmt.execute(params![email, hashed_password]).unwrap();

    Json(ApiResponse {
        response: true,
        message: "User created successfully".to_string(),
        data: Some(User {
            id: None,
            email,
            password: hashed_password,
        }),
    })
}

// Handle user login
#[post("/user/login", data = "<user>")]
async fn login_user(user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    let mut stmt = state.inner().prepare("SELECT id, password FROM users WHERE email = ?").unwrap();
    let mut rows = stmt.query(params![email]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let db_password: String = row.get(1).unwrap();
        if verify(&password, &db_password).unwrap() {
            let user_id: i32 = row.get(0).unwrap();
            let token = create_jwt(user_id);

            return Json(ApiResponse {
                response: true,
                message: "Login successful".to_string(),
                data: Some(User {
                    id: Some(user_id),
                    email,
                    password: token,
                }),
            });
        }
    }

    Json(ApiResponse {
        response: false,
        message: "Invalid credentials".to_string(),
        data: None,
    })
}

// Handle user update (example: password change)
#[put("/user/update/<id>", data = "<user>")]
async fn update_user(id: i32, user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    let hashed_password = hash_password(&password).await;
    let mut stmt = state.inner().prepare("UPDATE users SET email = ?, password = ? WHERE id = ?").unwrap();
    
    stmt.execute(params![email, hashed_password, id]).unwrap();

    Json(ApiResponse {
        response: true,
        message: "User updated successfully".to_string(),
        data: Some(User {
            id: Some(id),
            email,
            password: hashed_password,
        }),
    })
}

