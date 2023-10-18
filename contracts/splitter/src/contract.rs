use fixed_point_math::FixedPoint;
use soroban_sdk::{contract, contractimpl, contractmeta, token, Address, Env, Vec};

use crate::{
    errors::Error,
    storage::{ConfigDataKey, ShareDataKey},
};

contractmeta!(
    key = "desc",
    val = "Splitter contract is used to distribute tokens to shareholders with predefined shares."
);

pub trait SplitterTrait {
    fn init(env: Env, admin: Address, shares: Vec<ShareDataKey>) -> Result<(), Error>;

    fn distribute_tokens(env: Env, token_address: Address) -> Result<(), Error>;

    fn update_shares(env: Env, shares: Vec<ShareDataKey>);

    fn lock_contract(env: Env) -> Result<(), Error>;

    fn get_share(env: Env, shareholder: Address) -> Result<Option<i128>, Error>;

    fn get_config(env: Env) -> Result<ConfigDataKey, Error>;
}

#[contract]
pub struct Splitter;

#[contractimpl]
impl SplitterTrait for Splitter {
    fn init(env: Env, admin: Address, shares: Vec<ShareDataKey>) -> Result<(), Error> {
        if ConfigDataKey::exists(&env) {
            return Err(Error::AlreadyInitialized);
        };

        // Initialize the contract configuration
        ConfigDataKey::init(&env, admin, true);

        // Check if the shares sum up to 10000
        check_shares(&shares)?;

        // Shareholders are stored in a vector
        let mut shareholders: Vec<Address> = Vec::new(&env);

        // TODO: Check if the shares sum up to 10000
        // return an error if it doesn't

        for share in shares.iter() {
            // Add the shareholder to the vector
            shareholders.push_back(share.shareholder.clone());

            // Store the share for each shareholder
            ShareDataKey::save_share(&env, share.shareholder, share.share);
        }

        // Store the shareholders vector
        ShareDataKey::save_shareholders(&env, shareholders);

        Ok(())
    }

    fn distribute_tokens(env: Env, token_address: Address) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // TODO: Add admin check for unathorized access

        let token = token::Client::new(&env, &token_address);

        // Get the available token balance
        let balance = token.balance(&env.current_contract_address());

        // Get the shareholders vector
        let shareholders = ShareDataKey::get_shareholders(&env);

        // For each shareholder, calculate the amount of tokens to distribute
        for shareholder in shareholders.iter() {
            if let Some(ShareDataKey { share, .. }) = ShareDataKey::get_share(&env, &shareholder) {
                // Calculate the amount of tokens to distribute
                let amount = balance.fixed_mul_floor(share, 10000).unwrap_or(0);

                if amount > 0 {
                    // Transfer the tokens to the shareholder
                    token.transfer(&env.current_contract_address(), &shareholder, &amount);
                }
            };
        }

        Ok(())
    }

    fn update_shares(_env: Env, _shares: Vec<ShareDataKey>) {
        unimplemented!();
    }

    fn lock_contract(env: Env) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the sender is the admin
        let admin = ConfigDataKey::get(&env).unwrap().admin;
        admin.require_auth();

        // Update the contract configuration
        ConfigDataKey::lock_contract(&env);

        Ok(())
    }

    fn get_share(env: Env, shareholder: Address) -> Result<Option<i128>, Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };
        match ShareDataKey::get_share(&env, &shareholder) {
            Some(share) => Ok(Some(share.share)),
            None => Ok(None),
        }
    }

    fn get_config(env: Env) -> Result<ConfigDataKey, Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };
        Ok(ConfigDataKey::get(&env).unwrap())
    }
}

/// Checks if the shares sum up to 10000
fn check_shares(shares: &Vec<ShareDataKey>) -> Result<(), Error> {
    if shares.len() == 1 {
        return Err(Error::LowShareCount);
    };

    let total = shares.iter().fold(0, |acc, share| acc + share.share);

    if total != 10000 {
        return Err(Error::InvalidShareTotal);
    };

    Ok(())
}
