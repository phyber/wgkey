// Approximate of curve25519.* in wireguard-tools
use anyhow::Result;
use x25519_dalek::{
    PublicKey,
    StaticSecret,
};

pub fn clamp_secret(secret: &mut [u8]) {
    secret[0] &= 248;
    secret[31] = (secret[31] & 127) | 64;
}

pub fn generate_public(private_key: &[u8]) -> Result<Vec<u8>> {
    let private_key: [u8; 32] = private_key.try_into()?;
    let secret = StaticSecret::from(private_key);
    let public_key = PublicKey::from(&secret);

    Ok(public_key.to_bytes().to_vec())
}
