use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use solana_client::rpc_client::RpcClientConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::genesis_config::ClusterType::Devnet;
use solana_sdk::signature::{Keypair, keypair, Signer};
use solana_program::pubkey::Pubkey;

use solana_client::nonblocking::rpc_client::RpcClient;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let rpc_url: String = "https://api.devnet.solana.com/".to_string();

    let solana_rpc_client = RpcClient::new(rpc_url);

    // basic call to see if RPC works
    let latest_blockhash = solana_rpc_client.get_latest_blockhash().await?;
    println!("latest_blockhash: {:?}", latest_blockhash);

    // stefan mango account devnet: 5GHWjcYosrfPgfY3dS1itaWxSBs3veLtL4VMxj1EBLT5
    // random account on devnet: 2nUFxyZWH7RMwxQtXKwkxGH4EpKDHqXWZZ9ghRYShv6q
    let address = Pubkey::from_str("2nUFxyZWH7RMwxQtXKwkxGH4EpKDHqXWZZ9ghRYShv6q").unwrap();
    let transactions = solana_rpc_client.get_signatures_for_address(&address).await?;
    for tx in transactions {
        println!("tx: {:?}", tx);
    }

    Ok(())
}
