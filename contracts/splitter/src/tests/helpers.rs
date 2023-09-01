use crate::contract::{Splitter, SplitterClient};

use soroban_sdk::Env;

pub fn create_splitter<'a>(e: &Env) -> SplitterClient<'a> {
    let contract_id = e.register_contract(None, Splitter);
    let client = SplitterClient::new(&e, &contract_id);
    client
}
