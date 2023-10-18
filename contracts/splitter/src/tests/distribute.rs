use soroban_sdk::{testutils::Address as _, vec, Address, Env};

use crate::{
    errors::Error,
    storage::ShareDataKey,
    tests::helpers::{create_splitter, create_splitter_with_shares, create_token},
};

#[test]
fn happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);
    let shareholder_1 = Address::random(&env);
    let shareholder_2 = Address::random(&env);

    let (splitter, splitter_address) = create_splitter_with_shares(
        &env,
        &admin,
        &vec![
            &env,
            ShareDataKey {
                shareholder: shareholder_1.clone(),
                share: 8050,
            },
            ShareDataKey {
                shareholder: shareholder_2.clone(),
                share: 1950,
            },
        ],
    );

    let token_admin = Address::random(&env);
    let (token, sudo_token, token_address) = create_token(&env, &token_admin);

    sudo_token.mint(&splitter_address, &1_000_000_000);

    splitter.distribute_tokens(&token_address);

    assert_eq!(token.balance(&shareholder_1), 805_000_000);
    assert_eq!(token.balance(&shareholder_2), 195_000_000);
}

#[test]
fn test_not_initialized() {
    let env = Env::default();
    let (splitter, _) = create_splitter(&env);

    assert_eq!(
        splitter.try_distribute_tokens(&Address::random(&env)),
        Err(Ok(Error::NotInitialized))
    );
}
