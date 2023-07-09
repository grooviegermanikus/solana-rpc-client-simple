use anchor_lang::{AccountDeserialize, Id, Key};
use anchor_spl::token::Token;
use solana_program::instruction;
use solana_program::instruction::AccountMeta;

use std::cell::RefCell;
use std::{sync::Arc, sync::RwLock};
use std::ops::Deref;
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anchor_client::Cluster;
use clap::Parser;

use log::*;
use mango_v4::accounts_zerocopy::{AccountReader, LoadZeroCopy};
use mango_v4::state::{MangoAccountValue, MintInfo, PerpMarket, Serum3Market};
use solana_client::client_error::ClientError;
use solana_client::nonblocking::nonce_utils::get_account_with_commitment;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::{program_option::COption, program_pack::Pack};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use mango_v4_client::{AccountFetcher, Client, keypair_from_cli, MangoClient, Serum3MarketContext, TokenContext, TransactionBuilderConfig};

pub const MINT_ADDRESS_USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const MINT_ADDRESS_SOL: &str = "So11111111111111111111111111111111111111112";
pub const MINT_ADDRESS_ETH: &str = "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs";

#[derive(Parser, Debug, Clone)]
#[clap()]
struct Cli {
    // e.g. https://mango.devnet.rpcpool.com
    #[clap(short, long, env)]
    rpc_url: String,

    // from app mango -> "Accounts"
    #[clap(short, long, env)]
    mango_account: Pubkey,

    // path to json array with private key
    #[clap(short, long, env)]
    owner: String,
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let cli = Cli::parse_from(std::env::args_os());

    // --rpc-url https://api.devnet.solana.com/ --mango-account 5GHWjcYosrfPgfY3dS1itaWxSBs3veLtL4VMxj1EBLT5 --owner /Users/stefan/mango/solana-wallet/stefandev-devnet-keypair.json

    // --rpc-url http://mango.rpcpool.com/<token> --mango-account 7v8bovqsYfFfEeiXnGLiGTg2VJAn62hSoSCPidKjKL8w --owner /Users/stefan/mango/solana-wallet/solana-mainnet-stefantest.json


    let rpc_url = cli.rpc_url;
    let owner = Arc::new(keypair_from_cli(&cli.owner));
    let owner2 = keypair_from_cli(&cli.owner);
    let mango_account_pk = cli.mango_account;

    let ws_url = rpc_url.replace("https", "wss").replace("http", "ws");

    // group Czdh6uGt9x7EW7TAvN7ZwheSwYjiv29z6VD4yavkmHqe
    let cluster = Cluster::Custom(rpc_url.clone(), ws_url.clone());

    let client = Client::new(
        cluster,
        // TODO need two (ask Max)
        CommitmentConfig::confirmed(),
        owner.clone(),
        Some(Duration::from_secs(12)),
        TransactionBuilderConfig {
            prioritization_micro_lamports: Some(1),
        },
    );
    // build new client with the given config
    let solana_rpc_client = Arc::new(client.rpc_async());
    let mango_client = Arc::new(
        MangoClient::new_for_existing_account(
            client,
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

    for s3 in &mango_group_context.serum3_markets {
        println!("- serum3 market: {:?}", s3.1.market.name);
        println!("- serum3 market idx: {:?}", s3.1.market.market_index);
    }

    let serum3_info = mango_group_context.serum3_markets.get(&spot_market_index).unwrap();
    println!("serum3_info: {:?}", serum3_info.market);

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

    let solana_cookie = SolanaCookie::new_with_account_fetcher(mango_client.account_fetcher.clone());


    let account = mango_client.mango_account().await?;
    let account_fetcher = mango_client.account_fetcher.clone();


    let unique_client_order_id = get_epoch_micros();
    let ix = mango_client.make_raven_instruction(
        400_000_000, // .4 SOL
        unique_client_order_id,
        "SOL",
        "USDC",
        "SOL/USDC",
        "MNGO-PERP",
    ).await?;



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

impl TradeOpenbookInstruction {

    async fn to_instruction(&self, account_loader: impl ClientAccountLoader) {




    }

}


// copied from mango-v4
struct SolanaCookie {
    account_fetcher: Arc<dyn AccountFetcher>,
}

impl SolanaCookie {
    pub fn new_with_account_fetcher(account_fetcher: Arc<dyn AccountFetcher>) -> Self {
        SolanaCookie {
            account_fetcher,
        }
    }
}

impl SolanaCookie {

    pub async fn get_account_data(&self, address: Pubkey) -> Option<Vec<u8>> {
        Some(self.account_fetcher.fetch_raw_account(&address).await.unwrap().data().to_vec())
        // Some(
        //     get_account_with_commitment(&self.rpc_client, &address, CommitmentConfig::confirmed())
        //         .await.unwrap().data.to_vec(),
        // )
    }

}

#[async_trait::async_trait(?Send)]
pub trait ClientAccountLoader {
    async fn load_bytes(&self, pubkey: &Pubkey) -> Option<Vec<u8>>;
    async fn load<T: AccountDeserialize>(&self, pubkey: &Pubkey) -> Option<T> {
        let bytes = self.load_bytes(pubkey).await?;
        AccountDeserialize::try_deserialize(&mut &bytes[..]).ok()
    }
    async fn load_mango_account(&self, pubkey: &Pubkey) -> Option<MangoAccountValue> {
        self.load_bytes(pubkey)
            .await
            .map(|v| MangoAccountValue::from_bytes(&v[8..]).unwrap())
    }
}

#[async_trait::async_trait(?Send)]
impl ClientAccountLoader for &SolanaCookie {
    async fn load_bytes(&self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        self.get_account_data(*pubkey).await
    }
}


fn get_epoch_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

