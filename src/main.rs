#![deny(unused_crate_dependencies)]
use anyhow::Result;
use clap::Parser;
use std::{env, io};

mod cli;

const OS: &str = env::consts::OS;

fn main() -> Result<()> {
    let args = cli::WttuArgs::parse();
    let desired_outcome = &args.desired_outcome;

    let std_out = &mut io::stdout();

    // TODO:
    // have the mod error handle
    if let Err(e) = cli::find_cli_suggestons(desired_outcome, std_out, OS) {
        eprint!("{:?}", e)
    }

    Ok(())
}
