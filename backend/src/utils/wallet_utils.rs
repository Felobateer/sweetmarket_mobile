use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::utils::hex::ToHex;

struct WalletService {
    provider: Provider<Http>,
}

impl WalletService {
    fn new(rpc_url: &str) -> Self {
        let provider = Provider::<Http>::try_from(rpc_url)
            .expect("Failed to connect to RPC URL");
        WalletService { provider }
    }

    fn create_wallet(&self) -> (String, String) {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let private_key = wallet.signer().to_hex();
        let public_address = wallet.address().to_string();
        (private_key, public_address)
    }

    async fn get_balance(&self, address: &str) -> Result<U256, ProviderError> {
        let address: Address = address.parse().expect("Invalid address format");
        self.provider.get_balance(address, None).await
    }
}

#[tokio::main]
async fn main() {
    let rpc_url = "https://polygon-rpc.com";
    let service = WalletService::new(rpc_url);

    let (private_key, public_address) = service.create_wallet();
    println!("Private Key: {}", private_key);
    println!("Public Address: {}", public_address);

    match service.get_balance(&public_address).await {
        Ok(balance) => println!("Balance: {}", balance),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
}
