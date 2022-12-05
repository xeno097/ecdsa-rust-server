use std::str::FromStr;

use k256::ecdsa::{recoverable, Signature, VerifyingKey};
use sha3::Digest;

pub fn keccak256(message: impl AsRef<[u8]>) -> Vec<u8> {
    let mut hasher = sha3::Keccak256::new();

    hasher.update(message);

    hasher.finalize().into_iter().collect()
}

pub fn get_address(hex_public_key: &str) -> Result<String, String> {
    let byte_address = match hex::decode(hex_public_key) {
        Ok(value) => value,
        Err(_) => return Err(String::from("Invalid hex string")),
    };

    let (_, address) = byte_address.split_at(1);

    let hashed_pub_key = keccak256(address);

    let (_, partial_address) = hashed_pub_key.split_at(12);

    Ok(hex::encode(partial_address))
}

pub fn recover_key(message: &[u8], hex_signature: &str, recovery_id: u8) -> VerifyingKey {
    let raw_signature = Signature::from_str(hex_signature).unwrap();
    let parsed_recovery_id = recoverable::Id::new(recovery_id).unwrap();

    let signature = recoverable::Signature::new(&raw_signature, parsed_recovery_id).unwrap();

    // Do not hash the data again because by having the keccak256 flag enabled,
    // recover_verifying_key will hash the data for you to recover the pub key.
    signature.recover_verifying_key(message).unwrap()
}
