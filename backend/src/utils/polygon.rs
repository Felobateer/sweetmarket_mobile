use ethers::prelude::*;
use ethers::utils::hex::ToHex;
use std::sync::Arc;

pub struct Polygon {
    provider: Provider<Http>,
}

impl Polygon {
    pub fn new(rpc_url: &str) -> Self {
        let provider = Provider::<Http>::try_from(rpc_url)
            .expect("Failed to connect to Polygon RPC URL");
        Polygon { provider }
    }

    pub fn create_wallet(&self) -> (String, String) {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let private_key = wallet.signer().to_hex();
        let public_address = wallet.address().to_string();
        (private_key, public_address)
    }

    pub async fn get_balance(&self, address: &str) -> Result<U256, ProviderError> {
        let address: Address = address.parse().expect("Invalid address format");
        self.provider.get_balance(address, None).await
    }

    pub async fn transfer_native_token(
        &self,
        private_key: &str,
        to_address: &str,
        amount_in_wei: U256,
    ) -> Result<TxHash, Box<dyn std::error::Error>> {
        let signer = Wallet::from_str(private_key)?.connect(Arc::new(self.provider.clone()));
        let to: Address = to_address.parse()?;
        let tx = TransactionRequest::pay(to, amount_in_wei);
        let pending_tx = signer.send_transaction(tx, None).await?;
        Ok(pending_tx.tx_hash())
    }
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://polygon-rpc.com";
    let polygon = Polygon::new(rpc_url);

    // Create a wallet
    let (private_key, public_address) = polygon.create_wallet();
    println!("Private Key: {}", private_key);
    println!("Public Address: {}", public_address);

    // Fetch the balance
    match polygon.get_balance(&public_address).await {
        Ok(balance) => println!("Balance: {} WEI", balance),
        Err(err) => eprintln!("Error fetching balance: {}", err),
    }

    // Transfer tokens (example with 1 MATIC)
    let to_address = "0xRecipientAddressHere";
    let amount_in_wei = U256::exp10(18); // 1 MATIC in WEI
    match polygon.transfer_native_token(&private_key, to_address, amount_in_wei).await {
        Ok(tx_hash) => println!("Transaction sent! Hash: {:?}", tx_hash),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
