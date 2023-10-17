use soroban_sdk::{testutils::Address as _, vec, Address, Env};

use crate::{errors::Error, storage::ShareDataKey, tests::helpers::create_splitter};

#[test]
fn happy_path() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let admin = Address::random(&env);
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

    assert_eq!(splitter.try_init(&admin, &shares), Ok(Ok(())))
}

#[test]
fn test_already_initialized() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let admin = Address::random(&env);
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
    splitter.init(&admin, &shares);

    assert_eq!(
        splitter.try_init(&admin, &shares),
        Err(Ok(Error::AlreadyInitialized))
    );
}
