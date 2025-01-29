use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::extension::{Extension, ExtensionType};
use pqcrypto_dilithium::dilithium5::{verify, PublicKey, Signature};

#[derive(Clone, Debug, Default)]
struct PostQuantumExtension {
    post_quantum_pubkey: Vec<u8>, // Store the post-quantum public key
}

impl Extension for PostQuantumExtension {
    fn size() -> usize {
        2592 // Size for Dilithium5 public key
    }

    fn initialize(&mut self, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let post_quantum_pubkey_account = next_account_info(account_info_iter)?;

        // Load the post-quantum public key from the account data
        self.post_quantum_pubkey = post_quantum_pubkey_account.data.borrow().to_vec();
        Ok(())
    }

    fn verify_signature(&self, signature: &[u8], message: &[u8]) -> ProgramResult {
        let public_key = PublicKey::from_bytes(&self.post_quantum_pubkey).map_err(|_| ProgramError::InvalidArgument)?;
        let signature = Signature::from_bytes(signature).map_err(|_| ProgramError::InvalidArgument)?;

        if verify(message, &signature, &public_key) {
            Ok(())
        } else {
            msg!("Post-quantum signature verification failed");
            Err(ProgramError::InvalidArgument)
        }
    }
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse instruction data
    let instruction = instruction_data[0];
    match instruction {
        0 => {
            // Initialize the post-quantum extension
            let mut extension = PostQuantumExtension::default();
            extension.initialize(accounts)?;
        }
        1 => {
            // Verify a post-quantum signature
            let extension = PostQuantumExtension::default();
            let signature = &instruction_data[1..2593]; // Adjust based on signature size
            let message = &instruction_data[2593..];
            extension.verify_signature(signature, message)?;
        }
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
