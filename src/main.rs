#![deny(unused_crate_dependencies)]
use anyhow::Result;
use clap::Parser;
use std::{env, io};

mod cli;

const OS: &str = env::consts::OS;

fn main() -> Result<()> {
    let args = cli::WttuArgs::parse();
    let desired_outcome = &args.desired_outcome;
    let platform = &args.platform.unwrap_or(cli::underlining_os_to_enum(OS));

    let std_out = &mut io::stdout();

    // TODO:
    // have the mod error handle
    if let Err(e) = cli::find_cli_suggestons(desired_outcome, std_out, platform) {
        eprint!("{:?}", e)
    }

    Ok(())
}
