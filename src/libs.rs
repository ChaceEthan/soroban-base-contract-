#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

#[contract]
pub struct SorobanContract;

#[contractimpl]
impl SorobanContract {
    pub fn hello(env: Env, to: Symbol) -> Symbol {
        to
    }
}
