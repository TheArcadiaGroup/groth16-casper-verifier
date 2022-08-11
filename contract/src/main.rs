#![allow(unused_parens)]
#![allow(non_snake_case)]
#![no_main]

extern crate alloc;

use contract::{
    contract_api::{
        runtime,
        storage::{self, create_contract_package_at_hash},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use types::contracts::NamedKeys;
pub mod entry_points;
pub mod error;
pub mod final_exponentiation;
pub mod miller_loop;
pub mod pvk;
pub mod utils;

use crate::final_exponentiation::final_exponentiation_handler;
use crate::miller_loop::{delta_miller_loop_handler, gamma_miller_loop_handler};

#[no_mangle]
pub extern "C" fn gamma_miller_loop() {
    let i: u8 = runtime::get_named_arg("i");
    let j: u8 = runtime::get_named_arg("j");
    let input: Vec<u8> = runtime::get_named_arg("input");

    gamma_miller_loop_handler(i as usize, j as usize, input.as_slice());
}

#[no_mangle]
pub extern "C" fn delta_miller_loop() {
    let i: u8 = runtime::get_named_arg("i");
    let j: u8 = runtime::get_named_arg("j");
    let input: Vec<u8> = runtime::get_named_arg("input");

    delta_miller_loop_handler(i as usize, j as usize, input.as_slice());
}

#[no_mangle]
pub extern "C" fn final_exponentiation() {
    let i: u8 = runtime::get_named_arg("i");
    let j: u8 = runtime::get_named_arg("j");
    let input: Vec<u8> = runtime::get_named_arg("input");
    let keys: Vec<String> = runtime::get_named_arg("keys");

    final_exponentiation_handler(i as usize, j as usize, input.as_slice(), keys.as_slice());
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = entry_points::default();

    let data_seed_uref = storage::new_dictionary("data").unwrap_or_revert();

    let mut named_keys = NamedKeys::new();

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
    named_keys.insert("data".to_string(), data_seed_uref.into());

    named_keys.insert(
        "contract_package_hash".to_string(),
        storage::new_uref(contract_package_hash).into(),
    );

    // Add new version to the package.
    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);
    runtime::put_key(&"Verifier", contract_hash.into());
    runtime::put_key(&"Verifier_hash", storage::new_uref(contract_hash).into());
    runtime::put_key(&"Verifier_package_hash", contract_package_hash.into());
    runtime::put_key(&"Verifier_access_token", access_uref.into());
}
