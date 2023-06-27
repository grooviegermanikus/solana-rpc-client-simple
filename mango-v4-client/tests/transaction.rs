use std::str::FromStr;
use bincode::DefaultOptions;
use solana_client::rpc_client::SerializableTransaction;
use solana_sdk::bs58;
use solana_sdk::hash::{Hash, Hasher};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::{Transaction, VersionedTransaction};
use spl_memo::solana_program::message::VersionedMessage;

#[test]
fn build_tx_and_sign() {

    // https://solana.stackexchange.com/questions/22/how-to-generate-the-hash-of-a-transaction

    let payer = Keypair::from_base58_string("rKiJ7H5UUp3JR18kNyTF1XPuwPKHEM7gMLWHZPWP5djrW1vSjfwjhvJrevxF9MPmUmN9gJMLHZdLMgc9ao78eKr");
    let payer_pubkey = payer.pubkey();

    let memo_ix = spl_memo::build_memo("Hello world".as_bytes(), &[&payer_pubkey]);

    let mut tx = Transaction::new_with_payer(&[memo_ix], Some(&payer_pubkey));
    tx.sign(&[&payer], Hash::from_str("9HMCpfwpRh1TJ2E4NYb1P55tkfR9jQmwXkDBhHdFpudm").unwrap());
    let mut signature_base58_str = bs58::encode(tx.signatures[0]).into_string();

    assert_eq!(tx.signatures.len(), 1);
    assert_eq!(tx.is_signed(), true);
    assert_eq!(tx.get_recent_blockhash().to_string(), "9HMCpfwpRh1TJ2E4NYb1P55tkfR9jQmwXkDBhHdFpudm");
    assert_eq!(tx.get_signature().to_string(), "hqyiYYBhhoYxKrsaEzEUPPtaLTWALnxvYzijbWQqcXQKaFmFfaZsAPapK3HRoRF7a9Unn7oSXDFnVNDhYqJGFDr");
    assert_eq!(signature_base58_str, "hqyiYYBhhoYxKrsaEzEUPPtaLTWALnxvYzijbWQqcXQKaFmFfaZsAPapK3HRoRF7a9Unn7oSXDFnVNDhYqJGFDr");

}

#[test]
fn build_memo() {

    let payer_pubkey = Pubkey::from_str("Bm8rtweCQ19ksNebrLY92H7x4bCaeDJSSmEeWqkdCeop").unwrap();

    let memo_ix = spl_memo::build_memo("Hello world".as_bytes(), &[&payer_pubkey]);

    let tx = Transaction::new_with_payer(&[memo_ix], Some(&payer_pubkey));

    assert_eq!(tx.is_signed(), false);

}