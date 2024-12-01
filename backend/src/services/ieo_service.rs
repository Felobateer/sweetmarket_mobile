use reqwest::Client;

#[post("/create_crypto_payment", format = "json", data = "<crypto_request>")]
async fn create_crypto_payment(crypto_request: Json<CryptoPaymentRequest>) -> String {
    let client = Client::new();
    let response = client
        .post("https://www.coinpayments.net/api.php")
        .form(&[
            ("cmd", "create_transaction"),
            ("amount", &crypto_request.amount),
            ("currency1", &crypto_request.currency1),
            ("currency2", &crypto_request.currency2),
            ("buyer_email", &crypto_request.buyer_email),
            ("ipn_url", &crypto_request.ipn_url),
        ])
        .send()
        .await
        .unwrap();

    response.text().await.unwrap()
}
