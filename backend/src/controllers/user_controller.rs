#[macro_use] extern crate rocket;

use services::UserService;


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