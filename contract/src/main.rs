#![allow(unused_parens)]
#![allow(non_snake_case)]
#![no_main]

extern crate alloc;

use core::convert::TryInto;

use contract::{
    contract_api::{
        runtime,
        storage::{self, create_contract_package_at_hash},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    contracts::NamedKeys,
    system::CallStackElement,
    CLTyped, CLValue, Key, URef,
};
pub mod entry_points;
pub mod error;
use error::Error;

#[no_mangle]
pub extern "C" fn gamma_miller_loop() {
    let i: u8 = runtime::get_named_arg("i");
    let j: u8 = runtime::get_named_arg("j");
    let input: Vec<u8> = runtime::get_named_arg("input");
}

#[no_mangle]
pub extern "C" fn delta_miller_loop() {
    let i: u8 = runtime::get_named_arg("i");
    let j: u8 = runtime::get_named_arg("j");
    let input: Vec<u8> = runtime::get_named_arg("input");
}

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = entry_points::default();

    let mut named_keys = NamedKeys::new();

    let (contract_package_hash, access_uref) = create_contract_package_at_hash();
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

fn ret<T: CLTyped + ToBytes>(value: T) {
    runtime::ret(CLValue::from_t(value).unwrap_or_revert())
}

fn get_key<T: FromBytes + CLTyped + Default>(name: &str) -> T {
    match runtime::get_key(name) {
        None => Default::default(),
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            storage::read(key).unwrap_or_revert().unwrap_or_revert()
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

/// Returns the immediate caller address, whether it's an account or a contract.
fn get_caller() -> Key {
    let mut callstack = runtime::get_call_stack();
    callstack.pop();
    match callstack
        .last()
        .ok_or(Error::InvalidContext)
        .unwrap_or_revert()
    {
        CallStackElement::Session { account_hash } => (*account_hash).into(),
        CallStackElement::StoredSession {
            account_hash,
            contract_package_hash: _,
            contract_hash: _,
        } => (*account_hash).into(),
        CallStackElement::StoredContract {
            contract_package_hash,
            contract_hash: _,
        } => Key::from(*contract_package_hash),
    }
}
