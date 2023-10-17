use soroban_sdk::{testutils::Address as _, vec, Address, Env};

use crate::{
    storage::ShareDataKey,
    tests::helpers::{create_splitter_with_shares, create_token},
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

    assert_eq!(splitter.try_distribute_tokens(&token_address), Ok(Ok(())));

    assert_eq!(token.balance(&shareholder_1), 805_000_000);
    assert_eq!(token.balance(&shareholder_2), 195_000_000);
}
