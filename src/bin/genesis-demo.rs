extern crate serde_json;
extern crate silk;

use silk::mint::Mint;
use silk::event::Event;
use silk::transaction::Transaction;
use silk::entry::create_entry;
use silk::signature::{KeyPair, PublicKey};
use silk::hash::Hash;
use std::io::stdin;

fn transfer(from: &KeyPair, (to, tokens): (PublicKey, i64), last_id: Hash) -> Event {
    Event::Transaction(Transaction::new(&from, to, tokens, last_id))
}

fn main() {
    let mint: Mint = serde_json::from_reader(stdin()).unwrap();
    let mut entries = mint.create_entries();

    let from = mint.keypair();
    let seed = mint.seed();
    let alice = (KeyPair::new().pubkey(), 200);
    let bob = (KeyPair::new().pubkey(), 100);
    let events = vec![transfer(&from, alice, seed), transfer(&from, bob, seed)];
    entries.push(create_entry(&seed, 0, events));

    for entry in entries {
        println!("{}", serde_json::to_string_pretty(&entry).unwrap());
    }
}
