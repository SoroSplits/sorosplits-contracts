use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{contract, contractimpl, contractmeta, token, Address, Env, Vec};

use crate::{
    errors::Error,
    storage::{AllocationDataKey, ConfigDataKey, ShareDataKey},
};

contractmeta!(
    key = "desc",
    val = "Splitter contract is used to distribute tokens to shareholders with predefined shares."
);

pub trait SplitterTrait {
    /// Initializes the contract with the admin and the shareholders
    ///
    /// This method can only be called once.
    /// Runs the `check_shares` function to make sure the shares sum up to 10000.
    ///
    /// ## Arguments
    ///
    /// * `admin` - The admin address for the contract
    /// * `shares` - The shareholders with their shares
    /// * `mutable` - Whether the contract is mutable or not
    fn init(
        env: Env,
        admin: Address,
        shares: Vec<ShareDataKey>,
        mutable: bool,
    ) -> Result<(), Error>;

    // ========== Execute Functions ==========

    /// **ADMIN ONLY FUNCTION**
    ///
    /// Transfers unused tokens to the recipient.
    ///
    /// Unused tokens are defined as the tokens that are not distributed to the shareholders.
    /// Meaning token balance - sum of all the allocations.
    ///
    /// ## Arguments
    ///
    /// * `token_address` - The address of the token to transfer
    /// * `recipient` - The address of the recipient
    /// * `amount` - The amount of tokens to transfer
    fn transfer_tokens(
        env: Env,
        token_address: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<(), Error>;

    /// Distributes tokens to the shareholders.
    ///
    /// All of the available token balance is distributed on execution.
    ///
    /// ## Arguments
    ///
    /// * `token_address` - The address of the token to distribute
    fn distribute_tokens(env: Env, token_address: Address) -> Result<(), Error>;

    /// **ADMIN ONLY FUNCTION**
    ///
    /// Updates the shares of the shareholders.
    ///
    /// All of the shares and shareholders are updated on execution.
    ///
    /// ## Arguments
    ///
    /// * `shares` - The updated shareholders with their shares
    fn update_shares(env: Env, shares: Vec<ShareDataKey>) -> Result<(), Error>;

    /// **ADMIN ONLY FUNCTION**
    ///
    /// Locks the contract for further shares updates.
    ///
    /// Locking the contract does not affect the distribution of tokens.
    fn lock_contract(env: Env) -> Result<(), Error>;

    /// Withdraws the allocation of the shareholder for the token.
    ///
    /// A shareholder can withdraw their allocation for a token if they have any.
    ///
    /// ## Arguments
    ///
    /// * `token_address` - The address of the token to withdraw
    /// * `shareholder` - The address of the shareholder
    /// * `amount` - The amount of tokens to withdraw
    fn withdraw_allocation(
        env: Env,
        token_address: Address,
        shareholder: Address,
        amount: i128,
    ) -> Result<(), Error>;

    // ========== Query Functions ==========

    /// Gets the share of a shareholder.
    ///
    /// ## Arguments
    ///
    /// * `shareholder` - The address of the shareholder
    ///
    /// ## Returns
    ///
    /// * `Option<i128>` - The share of the shareholder if it exists
    fn get_share(env: Env, shareholder: Address) -> Result<Option<i128>, Error>;

    /// Lists all of the shareholders with their shares.
    ///
    /// ## Returns
    ///
    /// * `Vec<ShareDataKey>` - The list of shareholders with their shares
    fn list_shares(env: Env) -> Result<Vec<ShareDataKey>, Error>;

    /// Gets the contract configuration.
    ///
    /// ## Returns
    ///
    /// * `ConfigDataKey` - The contract configuration
    fn get_config(env: Env) -> Result<ConfigDataKey, Error>;

    /// Gets the allocation of a shareholder for a token.
    ///
    /// ## Arguments
    ///
    /// * `shareholder` - The address of the shareholder
    /// * `token` - The address of the token
    ///
    /// ## Returns
    ///
    /// * `i128` - The allocation of the shareholder for the token
    fn get_allocation(env: Env, shareholder: Address, token: Address) -> Result<i128, Error>;
}

#[contract]
pub struct Splitter;

#[contractimpl]
impl SplitterTrait for Splitter {
    fn init(
        env: Env,
        admin: Address,
        shares: Vec<ShareDataKey>,
        mutable: bool,
    ) -> Result<(), Error> {
        if ConfigDataKey::exists(&env) {
            return Err(Error::AlreadyInitialized);
        };

        // Initialize the contract configuration
        ConfigDataKey::init(&env, admin, mutable);

        // Check if the shares sum up to 10000
        check_shares(&shares)?;

        // Update the shares of the shareholders
        update_shares(&env, &shares);

        Ok(())
    }

    fn transfer_tokens(
        env: Env,
        token_address: Address,
        recipient: Address,
        amount: i128,
    ) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the caller is the admin
        ConfigDataKey::require_admin(&env)?;

        let token = token::Client::new(&env, &token_address);

        // Get the available token balance
        let balance = token.balance(&env.current_contract_address());

        // Get the total allocation for the token
        let total_allocation =
            AllocationDataKey::get_total_allocation(&env, &token_address).unwrap_or(0);

        // Calculate the unused balance that can be transferred
        let unused_balance = balance - total_allocation;

        // Transfer amount cannot be equal and less than 0
        if amount <= 0 {
            return Err(Error::ZeroTransferAmount);
        };
        // Transfer amount cannot be greater than the balance
        if amount > balance {
            return Err(Error::TransferAmountAboveBalance);
        };
        // Transfer amount cannot be greater than the unused balance
        if amount > unused_balance {
            return Err(Error::TransferAmountAboveUnusedBalance);
        };

        // Transfer the tokens to the recipient
        token.transfer(&env.current_contract_address(), &recipient, &amount);

        Ok(())
    }

    fn distribute_tokens(env: Env, token_address: Address) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the caller is the admin
        ConfigDataKey::require_admin(&env)?;

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
                    // Get the current allocation for the user - default to 0
                    let allocation =
                        AllocationDataKey::get_allocation(&env, &shareholder, &token_address)
                            .unwrap_or(0);

                    // Update the allocation with the new amount
                    AllocationDataKey::save_allocation(
                        &env,
                        &shareholder,
                        &token_address,
                        allocation + amount,
                    );
                }
            };
        }

        Ok(())
    }

    fn update_shares(env: Env, shares: Vec<ShareDataKey>) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the caller is the admin
        ConfigDataKey::require_admin(&env)?;

        // Check if the shares sum up to 10000
        check_shares(&shares)?;

        // Remove all of the shareholders and their shares
        reset_shares(&env);

        // Update the shares of the shareholders
        update_shares(&env, &shares);

        Ok(())
    }

    fn lock_contract(env: Env) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the caller is the admin
        ConfigDataKey::require_admin(&env)?;

        // Update the contract configuration
        ConfigDataKey::lock_contract(&env);

        Ok(())
    }

    fn withdraw_allocation(
        env: Env,
        token_address: Address,
        shareholder: Address,
        amount: i128,
    ) -> Result<(), Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        // Make sure the caller is the shareholder
        shareholder.require_auth();

        let token = token::Client::new(&env, &token_address);

        // Get the current allocation for the user - default to 0
        let allocation =
            AllocationDataKey::get_allocation(&env, &shareholder, &token_address).unwrap_or(0);

        // Withdraw amount cannot be equal and less than 0
        if amount == 0 {
            return Err(Error::ZeroWithdrawalAmount);
        };
        // Withdraw amount cannot be greater than the allocation
        if amount > allocation {
            return Err(Error::WithdrawalAmountAboveAllocation);
        };

        if amount == allocation {
            AllocationDataKey::remove_allocation(&env, &shareholder, &token_address);
        } else {
            AllocationDataKey::save_allocation(
                &env,
                &shareholder,
                &token_address,
                allocation - amount,
            );
        }

        // Transfer the tokens to the shareholder
        token.transfer(&env.current_contract_address(), &shareholder, &amount);

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

    fn list_shares(env: Env) -> Result<Vec<ShareDataKey>, Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };

        let mut shares: Vec<ShareDataKey> = Vec::new(&env);

        for shareholder in ShareDataKey::get_shareholders(&env).iter() {
            let share = ShareDataKey::get_share(&env, &shareholder).unwrap();
            shares.push_back(share);
        }

        Ok(shares)
    }

    fn get_config(env: Env) -> Result<ConfigDataKey, Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };
        Ok(ConfigDataKey::get(&env).unwrap())
    }

    fn get_allocation(env: Env, shareholder: Address, token: Address) -> Result<i128, Error> {
        if !ConfigDataKey::exists(&env) {
            return Err(Error::NotInitialized);
        };
        Ok(AllocationDataKey::get_allocation(&env, &shareholder, &token).unwrap_or(0))
    }
}

/// Updates the shares of the shareholders
fn update_shares(env: &Env, shares: &Vec<ShareDataKey>) {
    // Shareholders are stored in a vector
    let mut shareholders: Vec<Address> = Vec::new(&env);

    for share in shares.iter() {
        // Add the shareholder to the vector
        shareholders.push_back(share.shareholder.clone());

        // Store the share for each shareholder
        ShareDataKey::save_share(&env, share.shareholder, share.share);
    }

    // Store the shareholders vector
    ShareDataKey::save_shareholders(&env, shareholders);
}

/// Removes all of the shareholders and their shares
fn reset_shares(env: &Env) {
    for shareholder in ShareDataKey::get_shareholders(env).iter() {
        ShareDataKey::remove_share(env, &shareholder);
    }
    ShareDataKey::remove_shareholders(env);
}

/// Checks if the shares sum up to 10000
fn check_shares(shares: &Vec<ShareDataKey>) -> Result<(), Error> {
    if shares.len() < 2 {
        return Err(Error::LowShareCount);
    };

    let total = shares.iter().fold(0, |acc, share| acc + share.share);

    if total != 10000 {
        return Err(Error::InvalidShareTotal);
    };

    Ok(())
}
