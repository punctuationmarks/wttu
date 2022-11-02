#![deny(unused_crate_dependencies)]
use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::{env, fs};

// TODO:
// create help text and have autocomplete, where the user just types "enc" and then can hit tab to complete the enum
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(value_enum, default_value_t=DesiredOutcomes::WttuInfo)]
    // #[arg(short = 'o', long = "output")] // <- need to look into these, might be useful
    desired_outcome: DesiredOutcomes,
}

// these derive attributes are neccessary to pass an enum value as cli params
#[derive(ValueEnum, Clone, Debug)]
enum DesiredOutcomes {
    Checksum,
    CliGeneral,
    CliMeta,
    Compress,
    DocumentEditor,
    Editor,
    Encode,
    Encrypt,
    Find,
    Fingerprinting,
    Firewall,
    Images,
    List,
    Manual,
    Meta,
    // TODO: rename
    NavigateLocal,
    Networking,
    PackageManager,
    Partition,
    Permission,
    Print, // print to the console
    Privacy,
    RandomGenerator,
    System,
    VersionControl,
    WttuInfo,
}

// underlining operating system
const OS: &str = env::consts::OS;


fn main() -> Result<()> {
    let args = CliArgs::parse();

    if let Err(e) = find_cli_suggestons(&args.desired_outcome, &mut std::io::stdout(), OS) {
        eprint!("{:?}", e)
    }

    Ok(())
}

// TODO:
// extend this to have a json flag that will allow it to be piped into the next stage?
// need to think this through further
fn find_cli_suggestons(
    desired_outcome: &DesiredOutcomes,
    mut writer: impl std::io::Write,
    os: &str,
) -> Result<(), std::io::Error> {
    let json_output = create_json_output(os);
    let wtto_info = json!("wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.");
    let no_entry = json!(format!("No entry yet for that on {OS}, sorry. Feel free to open a PR for suggestions or email them to phoebx@pm.me or ironistdesign@pm.me"));


    let output = match desired_outcome {
        DesiredOutcomes::Checksum => json_output.get("checksum").unwrap_or(&no_entry),
        DesiredOutcomes::CliGeneral => json_output.get("cli_general").unwrap_or(&no_entry),
        DesiredOutcomes::CliMeta => json_output.get("cli_meta").unwrap_or(&no_entry),
        DesiredOutcomes::Compress => json_output.get("compress").unwrap_or(&no_entry),
        DesiredOutcomes::DocumentEditor => json_output.get("document_editor").unwrap_or(&no_entry),
        DesiredOutcomes::Editor => json_output.get("editor").unwrap_or(&no_entry),
        DesiredOutcomes::Encode => json_output.get("encode").unwrap_or(&no_entry),
        DesiredOutcomes::Encrypt => json_output.get("encrypt").unwrap_or(&no_entry),
        DesiredOutcomes::Find => json_output.get("find").unwrap_or(&no_entry),
        DesiredOutcomes::Fingerprinting => json_output.get("fingerprinting").unwrap_or(&no_entry),
        DesiredOutcomes::Firewall => json_output.get("firewall").unwrap_or(&no_entry),
        DesiredOutcomes::Images => json_output.get("images").unwrap_or(&no_entry),
        DesiredOutcomes::List => json_output.get("list").unwrap_or(&no_entry),
        DesiredOutcomes::Manual => json_output.get("manual").unwrap_or(&no_entry),
        DesiredOutcomes::Meta => json_output.get("meta").unwrap_or(&no_entry),
        DesiredOutcomes::NavigateLocal => json_output.get("navigate_local").unwrap_or(&no_entry),
        DesiredOutcomes::Networking => json_output.get("networking").unwrap_or(&no_entry),
        DesiredOutcomes::PackageManager => json_output.get("package_manager").unwrap_or(&no_entry),
        DesiredOutcomes::Partition => json_output.get("partition").unwrap_or(&no_entry),
        DesiredOutcomes::Permission => json_output.get("permissions").unwrap_or(&no_entry),
        DesiredOutcomes::Print => json_output.get("print").unwrap_or(&no_entry),
        DesiredOutcomes::Privacy => json_output.get("privacy").unwrap_or(&no_entry),
        DesiredOutcomes::RandomGenerator => json_output.get("random_generator").unwrap_or(&no_entry),
        DesiredOutcomes::System => json_output.get("system").unwrap_or(&no_entry),
        DesiredOutcomes::VersionControl => json_output.get("version_control").unwrap_or(&no_entry),

        // hard coded entrees for all platforms
        DesiredOutcomes::WttuInfo => &wtto_info
    };

    writeln!(writer, "{}", output)
}

fn create_json_output(os: &str) -> serde_json::Value {
    // TODO:
    // abstract this in such a way that it can be programatically added to.
    // I envision adding a suggestion is just running a function that updates the json structure
    // (but would this also allow updating the enum param possibilities?)

    let panic_msg = "create_json_output() - json data is malformed and pointing to the wrong platform. might need to open up the hood and fix the data's json file, or reach out to the dev";

    let json_output: serde_json::Value;
    if os == "windows" {
        let data = fs::read_to_string("./src/cli_tools/windows.json")
            .expect("unable to read json file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
        let platform = &json["platform"];
        let windows = json!("windows");

        if platform == &windows {
            json_output = json!(&json);
        } else {
            panic!("{}", panic_msg)
        }
    } else if os == "mac" {
        let data = fs::read_to_string("./src/cli_tools/mac.json")
            .expect("unable to read json file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
        let platform = &json["platform"];
        let mac = json!("mac");

        if platform == &mac {
            json_output = json!(&json);
        } else {
            panic!("{}", panic_msg)
        }

    // Linux being the default
    } else {
        let data = fs::read_to_string("./src/cli_tools/linux-general.json")
            .expect("unable to read json file");
        let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
        let platform = &json["platform"];
        let linux = json!("linux");

        if platform == &linux {
            json_output = json!(&json);
        } else {
            panic!("{}", panic_msg)
        }
    };

    json_output
}
#[test]
fn find_encode_suggeston() {
    // the writer
    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &"linux");
    assert_eq!(result, b"\"base64\"\n");


    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &"mac");
    assert_eq!(result, b"\"base64\"\n");

    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &"windows");
    assert_eq!(result, b"\"CertUtil\"\n");

}

#[test]
fn find_info_suggeston() {
    // the writer
    let wtto_byte_info :&[u8; 112]  = b"\"wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.\"\n";

    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::WttuInfo, &mut result, &"windows");
    assert_eq!(result, wtto_byte_info);
}
