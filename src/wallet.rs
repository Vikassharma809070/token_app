
use ic_cdk::export::Principal;
use ic_cdk_macros::{update, query};
use crate::token_contract::{transfer, balance_of};

#[update]
fn send_tokens(to: String, amount: u64) -> bool {
    let caller = ic_cdk::caller().to_string();
    transfer(caller, to, amount)
}

#[query]
fn get_balance() -> u64 {
    let caller = ic_cdk::caller().to_string();
    balance_of(caller)
}
