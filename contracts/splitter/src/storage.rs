use soroban_sdk::{contracttype, Address};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct ShareDataKey {
    pub shareholder: Address,
    pub share: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Shareholders,
    Share(Address),
}
