// Equivalent to pubkey.c from wireguard-tools
use crate::curve25519;
use anyhow::anyhow;
use anyhow::Result;
use base64ct::{
    Base64,
    Encoding,
};
use std::io;
use super::WG_KEY_LEN;

const WG_KEY_LEN_BASE64: usize = (((WG_KEY_LEN + 2) / 3) * 4) + 1;

// Generates a public key from a given private key.
fn generate_public_key(b64: &str) -> Result<String> {
    // Get the private key base64 and decode it
    let private_key = Base64::decode_vec(b64.trim_end())?;

    // Generate a public key based on the decoded private key, and encode it
    // as base64
    let public_key = curve25519::generate_public(&private_key)?;
    let public_key = Base64::encode_string(&public_key);

    Ok(public_key)
}

// Reads a base64 encoded private key from stdin and attempts to get a public
// key from it.
pub fn pubkey() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    if buffer.len() != WG_KEY_LEN_BASE64 {
        return Err(anyhow!("Key is not the correct length or format"));
    }

    let public_key = generate_public_key(&buffer)?;

    println!("{}", public_key);

    Ok(public_key)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_public_key_from_base64() {
        // Taken from `wg` output.
        let input    = "oOanTdwuPvcT0sRoLeJVDVKlIq9SQYsd8fKXC025gX0=";
        let expected = "1Nae12IQvpY6eZ6Wa9RjJ+ytyCrZruj6nLNIcjbuoXk=";

        let output = generate_public_key(&input).unwrap();

        assert_eq!(output, expected);
    }
}
