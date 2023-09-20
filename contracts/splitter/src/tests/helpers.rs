use soroban_sdk::{token, Address, Env, Vec};
use token::{StellarAssetClient as TokenAdminClient, Client as TokenClient};

use crate::{
    contract::{Splitter, SplitterClient},
    storage::ShareDataKey,
};

pub fn create_splitter(e: &Env) -> (SplitterClient, Address) {
    let contract_id = &e.register_contract(None, Splitter);
    (SplitterClient::new(&e, contract_id), contract_id.clone())
}

pub fn create_splitter_with_shares<'a>(
    e: &'a Env,
    shares: &Vec<ShareDataKey>,
) -> (SplitterClient<'a>, Address) {
    let (client, contract_id) = create_splitter(e);
    client.init(shares);
    (client, contract_id)
}

pub fn create_token<'a>(
    e: &Env,
    admin: &Address,
) -> (TokenClient<'a>, TokenAdminClient<'a>, Address) {
    let contract_id = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_id),
        TokenAdminClient::new(e, &contract_id),
        contract_id,
    )
}
