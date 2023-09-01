use soroban_sdk::{contract, contractimpl, Env, Vec, Address};

use crate::storage::{DataKey, ShareDataKey};

pub trait SplitterTrait {
    fn init(env: Env, shares: Vec<ShareDataKey>);

    fn distribute_tokens(env: Env);
}

#[contract]
pub struct Splitter;

#[contractimpl]
impl SplitterTrait for Splitter {
    fn init(env: Env, shares: Vec<ShareDataKey>) {
        // Shareholders are stored in a vector
        let mut shareholders: Vec<Address> = Vec::new(&env);

        for share in shares.iter() {
            // Add the shareholder to the vector
            shareholders.push_back(share.shareholder.clone());

            // Store the share for each shareholder
            env.storage()
                .persistent()
                .set(&DataKey::Share(share.shareholder), &share.share);
        }

        // Store the shareholders vector
        env.storage()
            .persistent()
            .set(&DataKey::Shareholders, &shareholders);
    }

    fn distribute_tokens(_env: Env) {}
}
