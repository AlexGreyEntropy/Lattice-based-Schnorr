use solana_program_test::*;
use solana_sdk::{signature::Signer, transaction::Transaction, signer::keypair::Keypair};
use post_quantum_extension::PostQuantumExtension;
use spl_token_2022::{instruction::initialize_account, state::Account};

#[tokio::test]
async fn test_post_quantum_extension() {
    // Set up the test environment
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "post_quantum_extension",
        program_id,
        processor!(process_instruction),
    );

    // Add the token extension program
    program_test.add_program(
        "spl_token_2022",
        spl_token_2022::id(),
        processor!(spl_token_2022::processor::Processor::process),
    );

    // Start the test
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create a new token account with the post-quantum extension
    let token_account = Keypair::new();
    let mint = Keypair::new();
    let post_quantum_pubkey = vec![0; 2592]; // Example Dilithium5 public key

    let transaction = Transaction::new_signed_with_payer(
        &[
            initialize_account(
                &spl_token_2022::id(),
                &token_account.pubkey(),
                &mint.pubkey(),
                &payer.pubkey(),
                &post_quantum_pubkey,
            ).unwrap(),
        ],
        Some(&payer.pubkey()),
        &[&payer, &token_account],
        recent_blockhash,
    );

    // Process the transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the post-quantum extension was initialized
    let account = banks_client.get_account(token_account.pubkey()).await.unwrap().unwrap();
    let extension = PostQuantumExtension::default();
    assert_eq!(account.data.len(), extension.size());
}
