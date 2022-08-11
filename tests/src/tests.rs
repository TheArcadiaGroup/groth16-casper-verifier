use casper_types::{account::AccountHash, Key};
use circuit::initialize;

use crate::contract::{Sender, Verifier};

fn to_key(account: AccountHash) -> Key {
    Key::Account(account)
}

#[test]
fn groth16_verify() {
    // run a circuit demo
    let (proof_c, prepared_input, qap) = initialize().unwrap();
    println!("run a circuit demo, get input and proof");

    let mut contract = Verifier::deployed();

    // gamma miller loop
    println!("running gamma miller loop");
    contract.gamma_miller_loop(prepared_input, contract.ali);

    // delta miller loop
    println!("running delta miller loop");
    contract.delta_miller_loop(proof_c, contract.bob);

    // // final exponentiation
    // println!("running final exponentiation");
    // contract.final_exponentiation(qap, contract.ali);
}
