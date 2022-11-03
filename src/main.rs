#![deny(unused_crate_dependencies)]
use anyhow::Result;
use clap::Parser;
use std::{env, io};

mod cli;

fn main() -> Result<()> {
    let args = cli::WttuArgs::parse();
    let desired_outcome = &args.desired_outcome;
    let platform = &args.platform.unwrap_or(underlining_os_to_enum());

    let std_out = &mut io::stdout();

    // TODO:
    // have the mod error handle
    if let Err(e) = cli::find_cli_suggestons(desired_outcome, std_out, platform) {
        eprint!("{:?}", e)
    }

    Ok(())
}


// these should be in a global helpers mod
fn underlining_os_to_enum() -> cli::SupportedPlatforms {
    let os: &str = env::consts::OS;
    match os {
        "linux" => cli::SupportedPlatforms::Linux,
        "mac" => cli::SupportedPlatforms::Linux,
        "windows" => cli::SupportedPlatforms::Linux,
        _ => cli::SupportedPlatforms::Unsupported,
    }
}
