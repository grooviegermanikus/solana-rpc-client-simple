use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, read_keypair_file, Signer};
use solana_sdk::sysvar;
use solana_sdk::transaction::Transaction;

fn main() {
    let rpc_client = RpcClient::new(
        "http://localhost:8899".to_string(),
    );

    let payer_keypair = read_keypair_file(Path::new("/Users/stefan/mango/code/solana_configure_local_cluster/validator_1/identity.json")).unwrap();
    let last_blockhash = rpc_client.get_latest_blockhash().unwrap();

    let am = AccountMeta::new(Pubkey::from_str("SysvarLastRestartS1ot1111111111111111111111").unwrap(), false);

    let program_id = Pubkey::from_str("3dCHNiByP1xvPQXDSNyhwggdKp15HPbChLnKAcaHPYth").unwrap();

    // sysvar program deployed to local cluster (solana_configure_local_cluster)
    // /Users/stefan/mango/code/solana-program-library/examples/rust/sysvar
    let instructions = vec![Instruction::new_with_bincode(program_id, &(), vec![])];

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer_keypair.pubkey()),
        &[&payer_keypair],
        last_blockhash,
    );

    rpc_client.send_and_confirm_transaction_with_spinner(&transaction).unwrap();


}
