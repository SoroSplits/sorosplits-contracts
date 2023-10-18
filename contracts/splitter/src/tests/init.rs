use soroban_sdk::{testutils::Address as _, vec, Address, Env};

use crate::{errors::Error, storage::ShareDataKey, tests::helpers::create_splitter};

#[test]
fn happy_path() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let admin = Address::random(&env);
    let shareholder_1 = Address::random(&env);
    let shareholder_2 = Address::random(&env);
    let shares = vec![
        &env,
        ShareDataKey {
            shareholder: shareholder_1.clone(),
            share: 8050,
        },
        ShareDataKey {
            shareholder: shareholder_2.clone(),
            share: 1950,
        },
    ];

    splitter.init(&admin, &shares);

    assert_eq!(splitter.get_share(&shareholder_1), Some(8050));
    assert_eq!(splitter.get_share(&shareholder_2), Some(1950));
    assert_eq!(splitter.list_shares(), shares);
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

#[test]
fn test_low_share_count() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let admin = Address::random(&env);
    let shares = vec![
        &env,
        ShareDataKey {
            shareholder: Address::random(&env),
            share: 8050,
        },
    ];

    assert_eq!(
        splitter.try_init(&admin, &shares),
        Err(Ok(Error::LowShareCount))
    );
}

#[test]
fn test_invalid_share_total() {
    let env: Env = Env::default();
    let (splitter, _) = create_splitter(&env);

    let admin = Address::random(&env);

    assert_eq!(
        splitter.try_init(
            &admin,
            &vec![
                &env,
                ShareDataKey {
                    shareholder: Address::random(&env),
                    share: 8050,
                },
                ShareDataKey {
                    shareholder: Address::random(&env),
                    share: 8050,
                },
            ]
        ),
        Err(Ok(Error::InvalidShareTotal))
    );

    assert_eq!(
        splitter.try_init(
            &admin,
            &vec![
                &env,
                ShareDataKey {
                    shareholder: Address::random(&env),
                    share: 8050,
                },
                ShareDataKey {
                    shareholder: Address::random(&env),
                    share: 50,
                },
            ]
        ),
        Err(Ok(Error::InvalidShareTotal))
    );
}
