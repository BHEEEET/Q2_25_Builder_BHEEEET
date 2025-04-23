mod programs;

#[cfg(test)] 
mod tests{
    use sha2::digest::core_api::UpdateCore;
    use solana_sdk::{
        message::Message, signature::{self, read_keypair_file, Keypair, Signer}, system_program, transaction::{self, Transaction}
    };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer,
    };
    use std::str::FromStr;

    use crate::programs::Turbin3_prereq::{CompleteArgs, TurbinePrereqProgram, UpdateArgs};
    #[test]
    fn enroll(){
        let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        let signer = read_keypair_file("../../../turbine-cohort-repo/0-pre-req/rust/Turbine-wallet.json").expect("Failed to read keypair file");

        println!("Signer: {}", signer.pubkey());

        let prereq = TurbinePrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Used update because i already had a pda from Q1
        let args = UpdateArgs {
            github: b"bheeeet".to_vec(),
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = TurbinePrereqProgram::update(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("Public Key: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    } 

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Failed to read keypair file");

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Succes! Signature:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }
            Err(err) => {
                println!("Airdrop failed: {}", err.to_string());
            }
        }
    } 
    
    #[test]
    fn transfer_sol() {
        let kp = read_keypair_file("dev-wallet.json").expect("Failed to read keypair file");
        let pk = kp.pubkey();

        let to_pk = Pubkey::from_str("GaKuQyYqJKNy8nN9Xf6VmYJQXzQDvvUHHc8kTeGQLL3f").unwrap();

        let rpc_client = RpcClient::new("https://api.devnet.solana.com");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("failed to get recent blockhash");


        let balance = rpc_client
        .get_balance(&kp.pubkey())
        .expect("failed to get balance");

        // calcualte fees

        let message = Message::new_with_blockhash(
            &[transfer(&pk,
             &to_pk, 
             balance
            )], Some(&kp.pubkey()), &recent_blockhash);

        // calculate exact fee
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("failed to get fee");

        // let message_bytes = b"I verify my solana Keypair";
        // let sig = kp.sign_message(message_bytes);
        // let is_valid = sig.verify(pk.as_ref(), message_bytes);

        // match is_valid {
        //     true => println!("Signature is valid!"),
        //     false => println!("Signature is invalid!"),
        // }


        let transaction = Transaction::new_signed_with_payer(&[transfer(
            &kp.pubkey(), 
            &to_pk, 
            balance - fee
        )], Some(&kp.pubkey()), &vec![&kp], recent_blockhash);

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("failed to send transaction");

            println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    }

}