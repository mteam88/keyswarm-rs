use ethers_core::rand::rngs::ThreadRng;

use ethers::prelude::*;
use ethers_core::rand::thread_rng;

#[tokio::main]
async fn main() {
    // Connect to the network
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap();

    // create multicall instance
    let mut multicall = Multicall::new(provider, None).await.unwrap();

    let mut rng = thread_rng();

    // create wallets with ethers
    let wallet = generate_wallet(&mut rng);

    let wallet2 = generate_wallet(&mut rng);

    let result: (U256, U256) = multicall
        .clear_calls()
        .add_get_eth_balance(wallet.address(), false)
        .add_get_eth_balance(wallet2.address(), false)
        .call()
        .await
        .unwrap();

    println!("Balance: {:?}", result);
}

fn generate_wallet(rng: &mut ThreadRng) -> LocalWallet {
    LocalWallet::new(rng)
}