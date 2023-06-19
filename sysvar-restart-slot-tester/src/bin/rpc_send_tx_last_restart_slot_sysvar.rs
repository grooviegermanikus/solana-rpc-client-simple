use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::example_mocks::solana_client::rpc_client::RpcClient;
use solana_sdk::genesis_config::ClusterType::Devnet;
use solana_sdk::signature::{Keypair, keypair, Signer};
use mango_v4_client::{Client, MangoClient, TransactionBuilderConfig};
use anchor_client::Cluster;


fn keypair_from_cli(keypair: &str) -> Keypair {
    let maybe_keypair = keypair::read_keypair(&mut keypair.as_bytes());
    match maybe_keypair {
        Ok(keypair) => keypair,
        Err(_) => {
            let path = std::path::PathBuf::from_str(keypair).unwrap();
            keypair::read_keypair_file(path)
                .unwrap_or_else(|_| panic!("Failed to read keypair from {}", keypair))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let rpc_url: String = "https://api.devnet.solana.com/".to_string();
    let ws_url = rpc_url.replace("https", "wss").replace("http", "ws");

    let owner: Arc<Keypair> = Arc::new(keypair_from_cli("/Users/stefan/mango/solana-wallet/solana-mainnet-stefantest.json"));


    // use private key (solana-keygen)
    let mango_account: Arc<Keypair> = Arc::new(Keypair::from_base58_string("7v8bovqsYfFfEeiXnGLiGTg2VJAn62hSoSCPidKjKL8w"));

    let cluster = Cluster::Custom(rpc_url.clone(), ws_url.clone());

    let mango_client = Arc::new(
        MangoClient::new_for_existing_account(
            Client::new(
                cluster,
                // TODO need two (ask Max)
                CommitmentConfig::processed(),
                owner.clone(),
                Some(Duration::from_secs(12)),
                TransactionBuilderConfig {
                    prioritization_micro_lamports: Some(1),
                },
            ),
            mango_account.pubkey(),
            owner.clone(),
        ).await?);

    Ok(())
}
