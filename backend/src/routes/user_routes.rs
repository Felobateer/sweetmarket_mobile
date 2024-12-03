use rocket::{post, put, routes, State};
use rocket::serde::json::Json;
use rusqlite::{params, Connection};
use crate::models::{UserSignUp, User, ApiResponse};
use crate::utils::{hash_password, verify_password, create_jwt};

// Handle user creation (sign-up)
#[post("/signup", data = "<user>")]
pub async fn create_user(user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    // Check if the user already exists
    let mut stmt = state.inner().prepare("SELECT id FROM users WHERE email = ?").unwrap();
    let mut rows = stmt.query(params![email]).unwrap();
    
    if rows.next().unwrap().is_some() {
        return Json(ApiResponse {
            response: false,
            message: "User already exists".to_string(),
            data: None,
        });
    }

    // Hash password and insert the user
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
#[post("/login", data = "<user>")]
pub async fn login_user(user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    // Fetch user details from the database
    let mut stmt = state.inner().prepare("SELECT id, password FROM users WHERE email = ?").unwrap();
    let mut rows = stmt.query(params![email]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let db_password: String = row.get(1).unwrap();
        if verify_password(&password, &db_password).unwrap() {
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
#[put("/update/<id>", data = "<user>")]
pub async fn update_user(id: i32, user: Json<UserSignUp>, state: &State<Connection>) -> Json<ApiResponse> {
    let email = user.email.clone();
    let password = user.password.clone();

    // Hash password and update user details
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

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, login_user, update_user]
}
