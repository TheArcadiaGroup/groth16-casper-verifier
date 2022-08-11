use casper_types::{bytesrepr::ToBytes, Key};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_time() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

pub fn key_to_str(key: &Key) -> String {
    let preimage = key.to_bytes().unwrap();
    base64::encode(&preimage)
}
