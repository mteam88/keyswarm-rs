use ethers_core::utils::hex::ToHex;

use ethers_core::rand::rngs::ThreadRng;

use ethers::prelude::*;
use ethers_core::rand::thread_rng;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


#[tokio::main]
async fn main() {
        // a builder for `FmtSubscriber`.
        let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let mut rng = thread_rng();

    loop {
        let start_time = std::time::Instant::now();

        let mut wallets = Vec::new();

        for _ in 0..1000 {
            wallets.push(generate_wallet(&mut rng));
        }

        let result = get_balances(wallets).await;

        for (wallet, balance) in result.iter() {
            if balance != &0.into() {
                info!("Address: {:#020x} has a balance of {}", wallet.address(), balance);
                info!("Private key: {}", get_pk_from_localwallet(wallet.clone()));
            }
        }

        info!("Done 1000 wallets in {:?} secs", start_time.elapsed());
    }
}

fn generate_wallet(rng: &mut ThreadRng) -> LocalWallet {
    LocalWallet::new(rng)
}

async fn get_balances(wallets: Vec<LocalWallet>) -> Vec<(LocalWallet, U256)> {
    let provider = Provider::<Http>::try_from("https://eth.llamarpc.com").unwrap();

    let mut multicall = Multicall::new(provider, None).await.unwrap();

    for wallet in wallets.iter() {
        multicall.add_get_eth_balance(wallet.address(), false);
    }

    let balances: Vec<U256> = multicall.call_array().await.unwrap();

    wallets
        .into_iter()
        .zip(balances.into_iter())
        .collect::<Vec<(LocalWallet, U256)>>()
}

fn get_pk_from_localwallet(wallet: LocalWallet) -> String {
    format!("0x{}", wallet.signer().to_bytes().encode_hex::<String>())
}
