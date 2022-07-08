// CLI parsing
use clap::{
    crate_description,
    crate_name,
    crate_version,
    ArgMatches,
    Command,
};

fn create_app<'a>() -> Command<'a> {
    let app = Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand_required(true);

    let genkey = Command::new("genkey")
        .about("Generates a new private key and writes it to stdout");

    let genpsk = Command::new("genpsk")
        .about("Generates a new preshared key and writes it to stdout");

    let pubkey = Command::new("pubkey")
        .about("Reads a private key from stdin and writes a public key to stdout");

    app
        .subcommand(genkey)
        .subcommand(genpsk)
        .subcommand(pubkey)
}

pub fn parse_args() -> ArgMatches {
    create_app().get_matches()
}
