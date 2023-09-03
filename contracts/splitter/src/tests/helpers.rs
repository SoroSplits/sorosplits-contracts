use crate::contract::{Splitter, SplitterClient};

use soroban_sdk::{token, Address, Env};
use token::AdminClient as TokenAdminClient;
use token::Client as TokenClient;

pub fn create_splitter(e: &Env) -> SplitterClient {
    SplitterClient::new(&e, &e.register_contract(None, Splitter))
}

pub fn create_token<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_id = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_id),
        TokenAdminClient::new(e, &contract_id),
    )
}
