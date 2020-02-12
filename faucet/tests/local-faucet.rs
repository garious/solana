use solana_faucet::faucet::{request_airdrop_transaction, run_local_faucet};
use solana_sdk::{
    hash::Hash,
    message::Message,
    pubkey::Pubkey,
    signature::{generate_keypair, KeypairUtil},
    system_instruction,
    transaction::Transaction,
};
use std::sync::mpsc::channel;

#[test]
fn test_local_faucet() {
    let keypair = generate_keypair();
    let to = Pubkey::new_rand();
    let lamports = 50;
    let blockhash = Hash::new(&to.as_ref());
    let create_instruction = system_instruction::transfer(&keypair.pubkey(), &to, lamports);
    let message = Message::new(vec![create_instruction]);
    let expected_tx = Transaction::new(&[&keypair], message, blockhash);

    let (sender, receiver) = channel();
    run_local_faucet(keypair, sender, None);
    let faucet_addr = receiver.recv().unwrap();

    let result = request_airdrop_transaction(&faucet_addr, &to, lamports, blockhash);
    assert_eq!(expected_tx, result.unwrap());
}
