#![no_std]

use soroban_sdk::{contractimpl, Env};

pub struct HelloWorld;

#[contractimpl]
impl HelloWorld {
    pub fn add(_env: Env, a: i32, b: i32) -> i32 {
            a + b
                }
                }