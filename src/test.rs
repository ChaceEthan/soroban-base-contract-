#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Symbol};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(SorobanContract, ());
    let client = SorobanContractClient::new(&env, &contract_id);

    let name = Symbol::from_str(&env, "Dev");
    let res = client.hello(&name);
    assert_eq!(res, name);
}
