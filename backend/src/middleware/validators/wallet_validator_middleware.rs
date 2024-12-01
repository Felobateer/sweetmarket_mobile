use rocket::serde::{Deserialize, json::Json};
use validator::{Validate, ValidationErrors};
use rocket::http::Status;
use rocket::response::content;

#[derive(Deserialize, Validate)]
pub struct CreateWallet {
    #[validate(length(min = 1, message = "KeyPhrase is required"))]
    pub keyphrase: String,
}

#[derive(Deserialize, Validate)]
pub struct AccessWalletWithPrivateKey {
    #[validate(length(min = 1, message = "PrivateKey is required"))]
    pub private_key: String,
}

#[derive(Deserialize, Validate)]
pub struct AddToken {
    #[validate(length(equal = 42, message = "PublicKey must contain 42 characters"))]
    pub public_key: String,

    #[validate(length(min = 1, message = "Token Symbol is required"))]
    pub token_symbol: String,

    #[validate(length(equal = 42, message = "Token Address must contain 42 characters"))]
    pub token_address: String,
}


#[catch(400)]
fn catch_validation_error() -> content::Json<String> {
    content::Json("{\"response\":false, \"message\":\"Validation error occurred\"}".to_string())
}

#[catch(400)]
fn catch_bad_request() -> String {
    String::from("{\"response\":false, \"message\":\"Bad Request\"}")
}
