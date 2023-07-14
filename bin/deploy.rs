use odra::{client_env};
use odra::types::Address;
use std::{str::FromStr};
// use hex::decode;

use something::some_contract::SomeContractDeployer; 

fn main() {
    client_env::set_gas(110_000_000_000u64);
    let mut gov_contract = SomeContractDeployer::init("test".to_string());

    client_env::set_gas(110_000_000_000u64);
    let res = gov_contract.add_something("some text".to_string());

    let smth = gov_contract.get_something(0);
    println!("Something: {:#?}", smth)
}