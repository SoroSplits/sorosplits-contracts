use soroban_sdk::{contract, contractimpl, Env, Vec};

use crate::storage::ShareDataKey;

pub trait SplitterTrait {
    fn init(env: Env, shares: Vec<ShareDataKey>);

    fn distribute_tokens(env: Env);
}

#[contract]
pub struct Splitter;

#[contractimpl]
impl SplitterTrait for Splitter {
    fn init(env: Env, shares: Vec<ShareDataKey>) {
        for share in shares.iter() {
            env.storage()
                .persistent()
                .set(&share.recipient, &share.share);
        }
    }

    fn distribute_tokens(env: Env) {}
}
