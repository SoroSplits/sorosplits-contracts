use soroban_sdk::{contract, contractimpl, Env, Vec};

use crate::storage::ShareDataKey;

pub trait SplitterTrait {
    fn init(env: Env, shares: Vec<ShareDataKey>);

    fn distribute_tokens();
}

#[contract]
pub struct Splitter;

#[contractimpl]
impl Splitter {}
