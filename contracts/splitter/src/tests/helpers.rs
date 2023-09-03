use soroban_sdk::{token, Address, Env, Vec};
use token::{AdminClient as TokenAdminClient, Client as TokenClient};

use crate::{
    contract::{Splitter, SplitterClient},
    storage::ShareDataKey,
};

pub fn create_splitter(e: &Env) -> SplitterClient {
    SplitterClient::new(&e, &e.register_contract(None, Splitter))
}

pub fn create_splitter_with_shares(e: &Env, shares: &Vec<ShareDataKey>) {
    let client = create_splitter(e);
    client.init(shares)
}

pub fn create_token<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_id = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_id),
        TokenAdminClient::new(e, &contract_id),
    )
}
