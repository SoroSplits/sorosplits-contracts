use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct ShareDataKey {
    pub recipient: Address,
    pub share: u64,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Share(Address),
}
