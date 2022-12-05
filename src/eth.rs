use std::str::FromStr;

use k256::{
    ecdsa::{recoverable, Signature},
    elliptic_curve::sec1::ToEncodedPoint,
};
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

// TODO: improve this implementation.
pub fn recover_key(message: &[u8], hex_signature: &str, recovery_id: u8) -> Result<String, String> {
    let error_message = Err(String::from("Could not recover the public key"));

    let raw_signature = match Signature::from_str(hex_signature) {
        Ok(value) => value,
        Err(_) => return error_message,
    };

    let parsed_recovery_id = match recoverable::Id::new(recovery_id) {
        Ok(recovery_bit) => recovery_bit,
        Err(_) => return error_message,
    };

    let signature = recoverable::Signature::new(&raw_signature, parsed_recovery_id).unwrap();

    // Do not hash the data again because by having the keccak256 flag enabled,
    // recover_verifying_key will hash the data for you to recover the pub key.
    let pub_key = match signature.recover_verifying_key(message) {
        Ok(pub_key) => pub_key,
        Err(_) => return error_message,
    };

    // Get the uncompressed public key
    let uncompressed_pub_key = pub_key.to_encoded_point(false);

    Ok(hex::encode(uncompressed_pub_key))
}
