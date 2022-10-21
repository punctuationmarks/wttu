use anyhow::Result;
use clap::{Parser, ValueEnum};
// use clap::Subcommand; // not entirely sure the usecase of a subcommand, but might be useful
use serde_json::json;

// TODO:
// create help text and have autocomplete, where the user just types "enc" and then can hit tab to complete the enum
#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(value_enum, default_value_t=DesiredOutcomes::Encrypt)]
    // // #[arg(short = 'o', long = "output")] // <- need to look into these, might be useful
    desired_outcome: DesiredOutcomes,
}

// these derive attributes are neccessary to pass an enum value as cli params
#[derive(ValueEnum, Clone, Debug)]
enum DesiredOutcomes {
    Compress,
    Encrypt,
}

// TODO:
// read the documentation for anyhow; this is returning a custom, anyhow Result
fn main() -> Result<()> {
    let args = CliArgs::parse();
    find_suggestons(&args.desired_outcome, &mut std::io::stdout());
    Ok(())
}

// TODO:
// extend this to have a json flag that will allow it to be piped into the next stage?
// need to think this through further
fn find_suggestons(desired_outcome: &DesiredOutcomes, mut writer: impl std::io::Write) -> () {
    // TODO:
    // abstract this in such a way that it can be programatically added to.
    // I envision adding a suggestion is just running a function that updates the json structure
    // (but would this also allow updating the enum param possibilities?)
    let json_output: serde_json::Value = json!({
        "compress": "gzip, zip, tar",
        "encrypt": "gpg"
    });

    // TODO:
    // this returns a result that doesn't handle all errors, need to read the book a bit more
    // to find out how to add error handling to match statements
    match desired_outcome {
        // two different ways to acess the field values
        DesiredOutcomes::Compress => writeln!(writer, "{}", *json_output.get("compress").unwrap()),
        DesiredOutcomes::Encrypt => writeln!(writer, "{}", json_output["encrypt"]),
        // no need for having _ since it's an enum
    };
}

#[test]
fn find_a_suggestons() {
    // the writer
    let mut result = Vec::new();
    find_suggestons(&DesiredOutcomes::Encrypt, &mut result);
    println!("---");
    println!("{:?}", result);
    println!("---");

    // this currently doesn't work with the json, I'm assuming it's
    // due to the serialization - since it's a byte operator due to the writer
    assert_eq!(result, b"gpg\n");
}

// NOTE:
// this is an old test that works, when the params were string literals (stack)
// leaving for just inspiration since they did work at one time. remove once the above
// tests are functional again.
// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("Encrypt", &mut result);
//     println!("---");
//     println!("{:?}", result);
//     println!("---");

//     assert_eq!(result, b"gpg\n");

//     // find_matches("encrypt", &mut result);
//     // assert_eq!(result, b"gpg\n");
// }
