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
            genkey(WG_KEY_LEN)?;
        },
        Some(("pubkey", _matches)) => {
            pubkey()?;
        },
        _ => unreachable!(),
    }

    Ok(())
}
