use soroban_sdk::{contracttype, Address};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct ShareDataKey {
    pub shareholder: Address,
    pub share: i128,
}

// TODO: Implement helper methods on ShareDataKey

// TODO: Add data key for contract configuration
// TODO: Implement helper methods

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Shareholders,
    Share(Address),
}
