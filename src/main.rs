use std::io::{self, Write};

use bitcoin::blockdata::transaction::{Transaction};
use bitcoin::consensus::encode::deserialize;
use hex::decode;

fn main() {
    // Read raw_transaction_hex from the terminal
    let raw_transaction_hex = read_user_input("Enter the raw transaction hex: ");


    // Decode the raw transaction hex
    let raw_transaction_bytes = decode(raw_transaction_hex).expect("Invalid hex");
    let transaction: Transaction = deserialize(&raw_transaction_bytes).expect("Failed to deserialize");

    // Display transaction details
    println!("Version: {}", transaction.version);

    println!("Inputs:");
    for input in &transaction.input {
        let prev_output = &input.previous_output;
        println!("  - Previous Output: {}", prev_output.txid);
        println!("    Index: {}", prev_output.vout);
    }

    println!("Outputs:");
    for (index, output) in transaction.output.iter().enumerate() {
        let value = output.value;
        let script_pubkey = &output.script_pubkey;
        
        println!("  - Index: {}", index);
        println!("    Value: {} Satoshis", value);
        println!("    Script: {}", script_pubkey.to_string());
    }


    println!("Locktime: {}", transaction.lock_time);
}


fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
