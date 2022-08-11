use casper_types::{account::AccountHash, Key};

use crate::contract::{Sender, Verifier};

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

#[test]
fn should_gamma_miller_loop() {
    let mut contract = Verifier::deployed();
    let input: Vec<u8> = vec![1, 2];
    contract.gamma_miller_loop(10, 20, input, Sender(contract.ali));
}
