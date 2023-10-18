use soroban_sdk::{contracttype, Address, Env, Vec};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct ShareDataKey {
    pub shareholder: Address,
    pub share: i128,
}
impl ShareDataKey {
    /// Initializes the share for the shareholder
    pub fn save_share(e: &Env, shareholder: Address, share: i128) {
        let key = DataKey::Share(shareholder.clone());
        e.storage()
            .persistent()
            .set(&key, &ShareDataKey { shareholder, share });
    }

    /// Returns the share for the shareholder
    pub fn get_share(e: &Env, shareholder: &Address) -> Option<ShareDataKey> {
        let key = DataKey::Share(shareholder.clone());
        e.storage().persistent().get(&key)
    }

    /// Saves the list of shareholders
    pub fn save_shareholders(e: &Env, shareholders: Vec<Address>) {
        e.storage()
            .persistent()
            .set(&DataKey::Shareholders, &shareholders);
    }

    /// Returns the list of shareholders
    pub fn get_shareholders(e: &Env) -> Vec<Address> {
        e.storage()
            .persistent()
            .get::<DataKey, Vec<Address>>(&DataKey::Shareholders)
            .unwrap_or(Vec::new(&e))
    }
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub struct ConfigDataKey {
    pub admin: Address,
    pub mutable: bool,
}
impl ConfigDataKey {
    /// Initializes the config with the given admin address and mutable flag
    pub fn init(e: &Env, admin: Address, mutable: bool) {
        let key = DataKey::Config;
        let config = ConfigDataKey { admin, mutable };
        e.storage().instance().set(&key, &config);
    }

    /// Returns the config
    pub fn get(e: &Env) -> Option<ConfigDataKey> {
        let key = DataKey::Config;
        e.storage().instance().get(&key)
    }

    /// Locks the contract for further changes
    pub fn lock_contract(e: &Env) {
        let key = DataKey::Config;
        let config: Option<ConfigDataKey> = e.storage().instance().get(&key);
        match config {
            Some(mut config) => {
                config.mutable = false;
                e.storage().instance().set(&key, &config);
            }
            None => (),
        }
    }

    /// Returns true if ConfigDataKey exists in the storage
    pub fn exists(e: &Env) -> bool {
        let key = DataKey::Config;
        e.storage().instance().has(&key)
    }

    /// Returns true if the address is admin
    // TODO: Maybe return an error if ConfigDataKey doesn't exist
    pub fn is_address_admin(e: &Env, address: Address) -> bool {
        let key = DataKey::Config;
        let config: Option<ConfigDataKey> = e.storage().instance().get(&key);
        match config {
            Some(config) => config.admin == address,
            None => false,
        }
    }

    /// Returns true if the contract is mutable
    // TODO: Maybe return an error if ConfigDataKey doesn't exist
    pub fn is_contract_locked(e: &Env) -> bool {
        let key = DataKey::Config;
        let config: Option<ConfigDataKey> = e.storage().instance().get(&key);
        match config {
            Some(config) => config.mutable,
            None => false,
        }
    }
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Shareholders,
    Share(Address),
}
