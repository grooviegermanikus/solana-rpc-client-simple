use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use solana_sdk::signature::{Keypair, keypair};


fn main() {


    // /Users/stefan/mango/solana-wallet/solana-mainnet-stefantest.json
    let owner: Arc<Keypair> = Arc::new(keypair_from_cli("/Users/stefan/mango/solana-wallet/solana-mainnet-stefantest.json"));



}

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

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//
//     let rpc_url = cli.rpc_url;
//     let ws_url = rpc_url.replace("https", "wss").replace("http", "ws");
//
//     // use private key (solana-keygen)
//     let owner: Arc<Keypair> = Arc::new(keypair_from_cli(cli.owner.as_str()));
//
//     let cluster = Cluster::Custom(rpc_url.clone(), ws_url.clone());
//
//     let mango_client = Arc::new(
//         MangoClient::new_for_existing_account(
//             Client::new(
//                 cluster,
//                 // TODO need two (ask Max)
//                 CommitmentConfig::processed(),
//                 owner.clone(),
//                 Some(Duration::from_secs(12)),
//                 TransactionBuilderConfig {
//                     prioritization_micro_lamports: Some(1),
//                 },
//             ),
//             cli.mango_account,
//             owner.clone(),
//         ).await?);
//
//
// }
