use solana_client::rpc_client::RpcClient;
use solana_program::example_mocks::solana_sdk::message::Message;
use solana_program::hash::hash;
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Signer, read_keypair_file};
use solana_sdk::transaction::Transaction;
use solana_system_interface::instruction::transfer;
use std::str::FromStr;

#[test]
fn transfer_sol() {
    // Load your devnet keypair from file
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Generate a signature from the keypair
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    // Verify the signature using the public key
    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification failed"),
    }

    let to_pubkey = Pubkey::from_str("<my Turbin3 public key>").unwrap();

    let rpc_client = RpcClient::new(crate::rpc::RPC_URL);

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
