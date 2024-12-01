#[macro_use] extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest::Client;

#[post("/create_payment_with_wallet", format = "json", data = "<payment_request>")]
async fn create_payment_with_wallet(payment_request: Json<WalletPaymentRequest>) -> String {
    let client = Client::new();
    let response = client
        .post("https://api.moonpay.com/v3/transactions")
        .json(&payment_request)
        .send()
        .await
        .unwrap();

    response.text().await.unwrap()
}


#[derive(Deserialize)]
struct PaymentDetails {
    amount: u64,
    currency: String,
    payment_method: String, // 'credit_card', 'crypto', 'google_pay', etc.
    payment_info: String,   // Details like token, crypto address, etc.
}

#[post("/process_payment", format = "json", data = "<payment_details>")]
async fn process_payment(payment_details: Json<PaymentDetails>) -> Json<String> {
    let response_message = match payment_details.payment_method.as_str() {
        "credit_card" => process_credit_card_payment(payment_details).await,
        "crypto" => process_crypto_payment(payment_details).await,
        "google_pay" | "apple_pay" => process_digital_wallet_payment(payment_details).await,
        _ => String::from("Invalid payment method."),
    };

    Json(response_message)
}

async fn process_credit_card_payment(payment_details: &Json<PaymentDetails>) -> String {
    // Call Stripe or PayPal API to process credit card payment
    format!("Processed credit card payment of {} {}", payment_details.amount, payment_details.currency)
}

async fn process_crypto_payment(payment_details: &Json<PaymentDetails>) -> String {
    // Call CoinPayments or MoonPay API to process cryptocurrency payment
    format!("Processed crypto payment of {} {}", payment_details.amount, payment_details.currency)
}

async fn process_digital_wallet_payment(payment_details: &Json<PaymentDetails>) -> String {
    // Call Google Pay or Apple Pay API
    format!("Processed payment with wallet for {} {}", payment_details.amount, payment_details.currency)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![process_payment])
}
