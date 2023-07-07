use anchor_lang::{Id, Key};
use anchor_spl::token::Token;
use solana_program::instruction;
use solana_program::instruction::AccountMeta;

use std::cell::RefCell;
use std::{sync::Arc, sync::RwLock};
use std::ops::Deref;
use std::str::FromStr;
use std::time::Duration;
use anchor_client::Cluster;

use log::*;
use solana_program::{program_option::COption, program_pack::Pack};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use mango_v4_client::{Client, keypair_from_cli, MangoClient, TokenContext, TransactionBuilderConfig};

pub const MINT_ADDRESS_USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const MINT_ADDRESS_SOL: &str = "So11111111111111111111111111111111111111112";
pub const MINT_ADDRESS_ETH: &str = "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs";


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let rpc_url: String = "https://api.devnet.solana.com/".to_string();
    let ws_url = rpc_url.replace("https", "wss").replace("http", "ws");

    let owner: Arc<Keypair> = Arc::new(keypair_from_cli("/Users/stefan/mango/solana-wallet/stefandev-devnet-keypair.json"));
    let owner2: Keypair = keypair_from_cli("/Users/stefan/mango/solana-wallet/stefandev-devnet-keypair.json");

    // Note: devnet / ckamm
    let mango_account_pk: Pubkey = Pubkey::from_str("5GHWjcYosrfPgfY3dS1itaWxSBs3veLtL4VMxj1EBLT5").unwrap();

    // group Czdh6uGt9x7EW7TAvN7ZwheSwYjiv29z6VD4yavkmHqe
    let cluster = Cluster::Custom(rpc_url.clone(), ws_url.clone());

    let mango_client = Arc::new(
        MangoClient::new_for_existing_account(
            Client::new(
                cluster,
                // TODO need two (ask Max)
                CommitmentConfig::confirmed(),
                owner.clone(),
                Some(Duration::from_secs(12)),
                TransactionBuilderConfig {
                    prioritization_micro_lamports: Some(1),
                },
            ),
            mango_account_pk,
            owner.clone(),
        ).await?);

    let mango_group_context = &mango_client.context;

    assert_eq!(mango_group_context.group.key(), Pubkey::from_str("Czdh6uGt9x7EW7TAvN7ZwheSwYjiv29z6VD4yavkmHqe").unwrap());

    for asdsdf in &mango_group_context.perp_market_indexes_by_name {
        println!("- perp market: {:?}", asdsdf);
    }

    // FIXME should be SOL-PERP
    let perp_market_index = mango_group_context.perp_market_indexes_by_name.get("MNGO-PERP").unwrap();
    let perp_market = mango_group_context.perp(*perp_market_index);

    let spot_market_index = mango_group_context
        .serum3_market_indexes_by_name
        .get("SOL/USDC")
        .unwrap();

    let serum3_info = mango_group_context.serum3_markets.get(&spot_market_index).unwrap();

    // TODO fill
    // let mango_account = Pubkey::new_unique();
    // let owner = Keypair::new();

    for tok in &mango_group_context.tokens {
        println!("- token: {:?} {}", tok.1.name, tok.1.mint_info_address);
    }

    let index_sol = mango_group_context.token_indexes_by_name.get("SOL").unwrap();
    let token_sol = mango_group_context.token(*index_sol);
    let index_usdc = mango_group_context.token_indexes_by_name.get("USDC").unwrap();
    let token_usdc = mango_group_context.token(*index_usdc);


    let ix = TradeOpenbookInstruction {
        client_order_id: 121212, // TODO
        base_native: 10_000_000, // .01 SOL
        account: mango_account_pk,
        owner: owner2, // payer / private key
        base_mint_info: Pubkey::from_str(MINT_ADDRESS_SOL).unwrap(),
        quote_mint_info: Pubkey::from_str(MINT_ADDRESS_USDC).unwrap(),
        perp_market: perp_market.address,
        serum_market: serum3_info.address,
        serum_market_external: serum3_info.market.serum_market_external,
        base_token_bank: token_sol.mint_info.first_bank(),
        quote_token_bank: token_usdc.mint_info.first_bank(),
    };

    let txsig = mango_client.send_and_confirm_owner_tx(vec![ix]).await;
    println!("txsig: {:?}", txsig);

    Ok(())
}


pub struct TradeOpenbookInstruction {
    // how much SOL we want to buy
    pub base_native: u64,
    // suggest to use epoch microseconds
    pub client_order_id: u64,

    pub quote_mint_info: Pubkey,
    pub base_mint_info: Pubkey,
    pub account: Pubkey,
    pub owner: Keypair,
    pub perp_market: Pubkey,
    // internals
    pub serum_market: Pubkey,
    pub serum_market_external: Pubkey,

    pub base_token_bank: Pubkey,
    pub quote_token_bank: Pubkey,
}

