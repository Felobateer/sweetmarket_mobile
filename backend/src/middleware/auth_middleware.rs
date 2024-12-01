use rocket::{Request, Data, Outcome, tokio};
use rocket::http::{Status, Cookie};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::Response;
use rocket::serde::{Deserialize, json::Json};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData};
use std::env;
use rocket::http::Header;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
struct Claims {
    user_id: String,
    role: String,
}

pub struct Auth;

#[rocket::async_trait]
impl Fairing for Auth {
    fn info(&self) -> Info {
        Info {
            name: "Auth Middleware",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        let auth_header = req.headers().get_one("Authorization");
        let bearer = "Bearer ";

        // Check if Authorization header exists and starts with "Bearer "
        if let Some(auth_header) = auth_header {
            if auth_header.starts_with(bearer) {
                let token = &auth_header[7..];

                // Decode the JWT
                let secret_key = env::var("SECRET_JWT").expect("SECRET_JWT must be set in .env");
                let validation = Validation {
                    algorithms: vec![Algorithm::HS256],
                    ..Validation::default()
                };

                match decode::<Claims>(&token, &DecodingKey::from_secret(secret_key.as_bytes()), &validation) {
                    Ok(decoded) => {
                        let user_id = decoded.claims.user_id;
                        let user_role = decoded.claims.role;

                        // Add custom logic here to check if the user exists or has the required permissions
                        // For example, check if user exists in the DB and whether the user is allowed to access the resource

                        // Placeholder: check if user_id matches requested ID or if the role is correct
                        if let Some(param_id) = req.get_param::<String>("id") {
                            if param_id != user_id && !check_permissions(&user_role, &req) {
                                req.local_cache(|| Response::new().set_status(Status::Unauthorized));
                                return;
                            }
                        }

                        // Store user info in request guard if necessary
                        req.local_cache(|| user_id);
                    }
                    Err(_) => {
                        req.local_cache(|| Response::new().set_status(Status::Unauthorized));
                    }
                }
            } else {
                req.local_cache(|| Response::new().set_status(Status::Unauthorized));
            }
        } else {
            req.local_cache(|| Response::new().set_status(Status::Unauthorized));
        }
    }
}

fn check_permissions(role: &str, req: &Request<'_>) -> bool {
    
    let allowed_roles = req.get_param::<Vec<String>>("roles");
    if allowed_roles.is_some() {
        return allowed_roles.unwrap().contains(&role.to_string());
    }
    false
}
