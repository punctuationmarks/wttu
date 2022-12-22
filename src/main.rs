// #![deny(unused_crate_dependencies)]
use clap::Parser;
use std::{env, io};

mod cli;

const OS: &str = env::consts::OS;

fn main() {
    let args = cli::WttuArgs::parse();
    let desired_outcome = &args.desired_outcome;
    let platform = &args
        .platform
        .unwrap_or_else(|| cli::underlining_os_to_enum(OS));
    let interface = &args.interface;
    let std_out = &mut io::stdout();

    match interface {
        Some(interface) => match interface {
            cli::Interface::Cli => cli::find_cli_suggestons(desired_outcome, std_out, platform),
            _ => todo!("Implement the Gui suggestions!"),
        },
        // default is cli
        None => cli::find_cli_suggestons(desired_outcome, std_out, platform),
    }
}
