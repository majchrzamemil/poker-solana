use poker_client::{
    check_balance, create_keypair, request_air_drop, send_transaction, transfer_funds,
};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use std::str::FromStr;
const URL: &str = "http://localhost:8899";

fn main() {
    let rpc_client = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = Pubkey::from_str("2sxavx4WRdB9UGi4Bx2dqZTEka4dYXBUaFYy14fJgS3D").unwrap(); //TODO: <-you know

    println!("Client: {:?}", sender.pubkey());

    let airdrop_signature = request_air_drop(&rpc_client, &sender.pubkey(), 1.0).unwrap();
    println!("Airdrop finished! Signature: {:?}", airdrop_signature);

    if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
        println!("Sender balance: {:?}", balance);
    }

    let transfer_amount = 0.5;

    match send_transaction(&rpc_client, &sender, &receiver, transfer_amount) {
        Ok(sig) => {
            println!("SUCCESS: {}", sig);
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
