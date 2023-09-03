use soroban_sdk::{testutils::Address as _, vec, Address, Env};

use crate::{storage::ShareDataKey, tests::helpers::create_splitter};

#[test]
fn happy_path() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let shares = vec![
        &env,
        ShareDataKey {
            shareholder: Address::random(&env),
            share: 8050,
        },
        ShareDataKey {
            shareholder: Address::random(&env),
            share: 1950,
        },
    ];

    splitter.init(&shares)
}
