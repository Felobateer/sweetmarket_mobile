#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::sync::Mutex;
use rocket::State;
use regex::Regex;
use std::sync::Arc;
use mongodb::{Client, Database};
use mongodb::bson::{doc, Bson};
use rocket::response::status;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Subscriber {
    pub email: String,
}

#[derive(Debug)]
pub struct SubscriberService {
    db: Arc<Mutex<Database>>,
}

#[rocket::async_trait]
impl SubscriberService {
    pub async fn create_subscriber(&self, subscriber: Subscriber) -> Result<String, String> {
        let email_validation = self.email_validation(&subscriber.email);
        if !email_validation {
            return Err("Email validation failed.".to_string());
        }

        let db = self.db.lock().await;
        let collection = db.collection::<Subscriber>("subscribers");
        let existing = collection.find_one(doc! { "email": &subscriber.email }, None).await.unwrap();

        if existing.is_some() {
            return Err("Already registered.".to_string());
        }

        collection.insert_one(subscriber.clone(), None).await.unwrap();
        Ok("Subscriber created successfully!".to_string())
    }

    pub async fn get_all_subscribers(&self) -> Result<Vec<Subscriber>, String> {
        let db = self.db.lock().await;
        let collection = db.collection::<Subscriber>("subscribers");

        let cursor = collection.find(None, None).await.unwrap();

        let mut subscribers: Vec<Subscriber> = Vec::new();

        for result in cursor {
            match result {
                Ok(subscriber) => subscribers.push(subscriber),
                Err(_) => return Err("Error retrieving subscribers.".to_string()),
            }
        }

        Ok(subscribers)
    }

    fn email_validation(&self, email: &str) -> bool {
        let re = Regex::new(r"^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$").unwrap();
        re.is_match(email)
    }
}

#[post("/subscribe", data = "<subscriber>")]
async fn create_subscriber(subscriber: Json<Subscriber>, service: &State<SubscriberService>) -> Json<Result<String, String>> {
    let result = service.create_subscriber(subscriber.into_inner()).await;
    Json(result.map_err(|e| e))
}

#[get("/subscribers")]
async fn get_all_subscribers(service: &State<SubscriberService>) -> Json<Result<Vec<Subscriber>, String>> {
    let result = service.get_all_subscribers().await;
    Json(result.map_err(|e| e))
}

#[launch]
fn rocket() -> _ {
    let client = Client::with_uri_str("mongodb://localhost:27017").unwrap();
    let db = client.database("subscriber_db");

    rocket::build()
        .manage(SubscriberService { db: Arc::new(Mutex::new(db)) })
        .mount("/", routes![create_subscriber, get_all_subscribers])
}
