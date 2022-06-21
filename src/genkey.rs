// Equivalent to genkey.c from wireguard-tools
use crate::curve25519;
use anyhow::{
    anyhow,
    Result,
};
use base64ct::{
    Base64,
    Encoding,
};
use rand::prelude::*;

const MAX_RANDOM_BYTES_LEN: usize = 256;

pub fn get_random_bytes(len: usize) -> Result<Vec<u8>> {
    if len > MAX_RANDOM_BYTES_LEN {
        let msg = anyhow!(
            "get_random_bytes length cannot exceed {}",
            MAX_RANDOM_BYTES_LEN,
        );

        return Err(msg);
    }

    let mut bytes: Vec<u8> = vec![0; len];
    thread_rng().fill_bytes(&mut bytes);

    Ok(bytes.to_vec())
}

pub fn genkey(len: usize) -> Result<String> {
    let mut bytes = get_random_bytes(len)?;

    curve25519::clamp_secret(&mut bytes);

    let encoded = Base64::encode_string(&bytes);

    Ok(encoded)
}
