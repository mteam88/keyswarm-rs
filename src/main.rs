use ethers_core::utils::hex::ToHex;
use std::collections::HashMap;

use ethers_core::rand::rngs::ThreadRng;

use ethers::prelude::*;
use ethers_core::rand::thread_rng;

#[tokio::main]
async fn main() {
    let mut rng = thread_rng();

    // create wallets with ethers
    let wallet = generate_wallet(&mut rng);

    let wallet2 = generate_wallet(&mut rng);

    let result = get_balances(vec![wallet.address(), wallet2.address()]).await;

    for (address, balance) in result.iter() {
        if balance == &0.into() {
            println!("Address: {:#020x} has a balance of {}", address, balance);
            println!("Private key: {}", get_pk_from_localwallet(wallet.clone()));
        }
    }
}

fn generate_wallet(rng: &mut ThreadRng) -> LocalWallet {
    LocalWallet::new(rng)
}

async fn get_balances(wallets: Vec<Address>) -> HashMap<Address, U256> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap();

    let mut multicall = Multicall::new(provider, None).await.unwrap();

    for wallet in wallets.iter() {
        multicall.add_get_eth_balance(*wallet, false);
    }

    let balances: Vec<U256> = multicall.call_array().await.unwrap();

    wallets
        .into_iter()
        .zip(balances.into_iter())
        .collect::<HashMap<Address, U256>>()
}

fn get_pk_from_localwallet(wallet: LocalWallet) -> String {
    format!("0x{}", wallet.signer().to_bytes().encode_hex::<String>())
}
