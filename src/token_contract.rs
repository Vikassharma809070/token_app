
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[derive(Default)]
struct Token {
    balances: HashMap<String, u64>,
    last_transaction: HashMap<String, u64>, // Tracks the last transaction time
}

const TRANSFER_LIMIT_SECONDS: u64 = 60; // 1 minute

#[update]
fn transfer(from: String, to: String, amount: u64) -> bool {
    let mut token = Token::default();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let last_tx_time = token.last_transaction.entry(from.clone()).or_insert(0);

    if current_time - *last_tx_time < TRANSFER_LIMIT_SECONDS {
        return false; // Rate limit reached
    }

    let sender_balance = token.balances.entry(from.clone()).or_default();
    if *sender_balance >= amount {
        *sender_balance -= amount;
        let receiver_balance = token.balances.entry(to.clone()).or_default();
        *receiver_balance += amount;
        token.last_transaction.insert(from, current_time); // Update last transaction time
        true
    } else {
        false
    }
}
