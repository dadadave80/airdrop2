use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_sdk::transaction::Transaction;
use solana_system_interface::program as system_program;
use std::str::FromStr;

#[test]
fn enroll() {
    let rpc_client = RpcClient::new(crate::rpc::RPC_URL);

    let turbin3_prereq_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();

    let system_program = system_program::id();

    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
    let mint = Keypair::new();

    let signer_pubkey = signer.pubkey();
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

    let authority_seeds = &[b"collection", collection.as_ref()];
    let (authority, _bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);

    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),     // user signer
        AccountMeta::new(prereq_pda, false),         // PDA account
        AccountMeta::new(mint.pubkey(), true),       // mint keypair
        AccountMeta::new(collection, false),         // collection
        AccountMeta::new_readonly(authority, false), // authority (PDA)
        AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
        AccountMeta::new_readonly(system_program, false), // system program
    ];

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
