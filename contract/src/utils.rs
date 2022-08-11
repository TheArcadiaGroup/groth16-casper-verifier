use ark_bn254::Fq12Parameters;
use ark_ff::{to_bytes, Fp12, Fp12ParamsWrapper, FromBytes, QuadExtField};
use arrayref::{array_mut_ref, array_ref};
use contract::{
    contract_api::{
        runtime,
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use num_traits::One;
use types::{
    bytesrepr::{FromBytes as CasperFromBytes, ToBytes},
    CLTyped, Key, URef,
};

pub const BN254_DATA_LEN: usize = 384;

pub fn get_account_data(key: String, j: usize) -> QuadExtField<Fp12ParamsWrapper<Fq12Parameters>> {
    let f = match j {
        0 => Fp12::<Fq12Parameters>::one(),
        _ => {
            let src: Vec<u8> = get("data", &key).unwrap_or([0u8; BN254_DATA_LEN].to_vec());
            let src = array_ref![src, 0, BN254_DATA_LEN];
            Fp12::<Fq12Parameters>::read(&mut src.as_ref()).unwrap()
        }
    };
    f
}

pub fn put_account_data(key: String, f: &QuadExtField<Fp12ParamsWrapper<Fq12Parameters>>) {
    let mut dst: Vec<u8> = get("data", &key).unwrap_or([0u8; BN254_DATA_LEN].to_vec());
    let dst = array_mut_ref![dst, 0, BN254_DATA_LEN];
    dst.copy_from_slice(to_bytes!(f).unwrap().as_slice());

    set("data", &key, dst.to_vec());
}

fn get<T: CasperFromBytes + CLTyped + Default>(dictionary_name: &str, key: &str) -> Option<T> {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_get(dictionary_seed_uref, key).unwrap_or_default()
}

fn set<T: ToBytes + CLTyped>(dictionary_name: &str, key: &str, value: T) {
    let dictionary_seed_uref = get_dictionary_seed_uref(dictionary_name);
    storage::dictionary_put(dictionary_seed_uref, key, value)
}

fn get_dictionary_seed_uref(name: &str) -> URef {
    match runtime::get_key(name) {
        Some(key) => key.into_uref().unwrap_or_revert(),
        None => {
            let new_dict = storage::new_dictionary(name).unwrap_or_revert();
            let key = storage::new_uref(new_dict).into();
            runtime::put_key(name, key);
            new_dict
        }
    }
}

fn key_to_str(key: &Key) -> String {
    let preimage = key.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}
