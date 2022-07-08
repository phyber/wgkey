// wgkey: Generate WireGuard compatible public/private keys
use anyhow::Result;

mod cli;
mod curve25519;
mod genkey;
mod pubkey;

use genkey::genkey;
use pubkey::pubkey;

const WG_KEY_LEN: usize = 32;

fn main() -> Result<()> {
    let args = cli::parse_args();

    match args.subcommand() {
        Some(("genkey", _matches)) => {
            let private_key = genkey(WG_KEY_LEN)?;

            println!("{}", private_key);
        },
        Some(("genpsk", _matches)) => {
            let psk = genkey(WG_KEY_LEN)?;

            println!("{}", psk);
        },
        Some(("pubkey", _matches)) => {
            let public_key = pubkey()?;

            println!("{}", public_key);
        },
        _ => unreachable!(),
    }

    Ok(())
}
