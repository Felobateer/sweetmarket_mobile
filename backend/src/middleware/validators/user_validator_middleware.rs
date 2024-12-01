use rocket::serde::{Deserialize, json::Json};
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(email(message = "Must be a valid email"))]
    #[validate(length(max = 50, message = "Email should contain less than 50 characters"))]
    pub email: String,

    #[validate(length(min = 4, max = 4, message = "Invite code should contain 4 characters"))]
    #[validate(regex = "^[0-9]+$", message = "Invite code must contain numeric characters")]
    pub invite_code: String,

    #[validate(length(min = 8, message = "Password must contain at least 8 characters"))]
    pub password: String,

    #[validate(custom = "validate_password_confirm")]
    pub confirm_password: String,

    #[validate(length(min = 3, message = "Country should contain at least 3 characters"))]
    pub country: String,
}

fn validate_password_confirm(confirm_password: &str, user: &User) -> Result<(), String> {
    if confirm_password != user.password {
        return Err("Confirm password must be the same as password".to_string());
    }
    Ok(())
}
