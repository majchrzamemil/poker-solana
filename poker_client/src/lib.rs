use serde::ser::StdError;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_program::message::Message;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::{
    signature::{Keypair, Signature},
    system_transaction,
};
use std::error::Error;

const LAMPORTS_PER_SOL: f64 = 1000000000.0;

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_air_drop(
    rpc_client: &RpcClient,
    pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_funds(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;

    Ok(
        rpc_client.send_and_confirm_transaction(&system_transaction::transfer(
            &sender_keypair,
            &receiver_pub_key,
            amount_lamports,
            rpc_client.get_latest_blockhash()?,
        ))?,
    )
}

#[derive(Serialize, Deserialize)]
pub struct BuyIn {
    pub stack: u64,
}

pub fn create_instruction(
    program_id: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    lamports: u64,
) -> Instruction {
    let instr = BuyIn { stack: lamports };

    Instruction::new_with_bincode(
        *program_id,
        &instr,
        vec![AccountMeta::new(*from, true), AccountMeta::new(*to, false)],
    )
}
pub fn send_transaction(
    rpc_client: &RpcClient,
    sender_keypair: &Keypair,
    receiver_pub_key: &Pubkey,
    amount_sol: f64,
) -> Result<solana_sdk::signature::Signature, solana_client::client_error::ClientError> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;
    let instruction = create_instruction(
        receiver_pub_key,
        &sender_keypair.pubkey(),
        receiver_pub_key,
        amount_lamports,
    );

    let message = Message::new(&[instruction], Some(&sender_keypair.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    let blockhash = rpc_client.get_latest_blockhash()?;
    tx.sign(&[sender_keypair], blockhash);
    rpc_client.send_and_confirm_transaction(&tx)
}
