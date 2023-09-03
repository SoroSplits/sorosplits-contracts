use fixed_point_math::{FixedPoint, STROOP};
use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::storage::{DataKey, ShareDataKey};

pub trait SplitterTrait {
    fn init(env: Env, shares: Vec<ShareDataKey>);

    fn distribute_tokens(env: Env, token_address: Address);
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

    fn distribute_tokens(env: Env, token_address: Address) {
        let token = token::Client::new(&env, &token_address);

        // Get the available token balance
        let balance = token.balance(&env.current_contract_address());

        // Get the shareholders vector
        let shareholders = env
            .storage()
            .persistent()
            .get::<DataKey, Vec<Address>>(&DataKey::Shareholders)
            .unwrap_or(Vec::new(&env));

        // For each shareholder, calculate the amount of tokens to distribute
        for shareholder in shareholders.iter() {
            let share = env
                .storage()
                .persistent()
                .get::<DataKey, i128>(&DataKey::Share(shareholder.clone()))
                .unwrap_or(0);

            // Calculate the amount of tokens to distribute
            let amount = balance.fixed_mul_floor(share, STROOP.into()).unwrap_or(0);

            // Transfer the tokens to the shareholder
            token.transfer(&env.current_contract_address(), &shareholder, &amount);
        }
    }
}
