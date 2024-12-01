#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use stripe::{Client, CreatePaymentIntent, PaymentIntent};

#[derive(Deserialize)]
struct PaymentRequest {
    amount: i64,
    currency: String,
}

#[post("/create_payment_intent", format = "json", data = "<payment_request>")]
async fn create_payment_intent(payment_request: Json<PaymentRequest>) -> Json<PaymentIntent> {
    let client = Client::new("sk_test_your_secret_key");
    let intent = CreatePaymentIntent {
        amount: payment_request.amount,
        currency: &payment_request.currency,
        ..Default::default()
    };
    
    match client.payment_intents().create(intent).await {
        Ok(payment_intent) => Json(payment_intent),
        Err(e) => {
            println!("Error: {}", e);
            Json(PaymentIntent::default())
        }
    }
}
