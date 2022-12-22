// use anyhow::Result;
use clap::{Parser, ValueEnum};
use serde_json::json;
use std::{fs};

/**
 * This mod creates Command Line suggestions. If extending to GUI, protocol, paradigm, language, etc etc, create addition mods and update accordingly.
 *
 * TODO:
 * Move all shared logic to another mod, so it will be easier to extend,
 */

// TODO:
// create help text and have autocomplete, where the user just types "enc" and then can hit tab to complete the enum
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct WttuArgs {
    #[clap(value_enum, default_value_t=DesiredOutcomes::WttuInfo)]
    pub desired_outcome: DesiredOutcomes,
    // TODO, implement this, will require refactor create_json_output to be match statement and maybe more
    // need to also look into how clap handles multiple arguments and how to make them optional
    #[clap(value_enum)]
    pub platform: Option<SupportedPlatforms>,
}

// TODO:
// is there a way to pass these as shorthand? -l for linux, -m for mac, etc
#[derive(Clone, Debug, ValueEnum)]
pub enum SupportedPlatforms {
    Linux,
    Mac,
    Windows,
    Unsupported,
}

// these derive attributes are neccessary to pass an enum value as cli params
#[derive(ValueEnum, Clone, Debug)]
pub enum DesiredOutcomes {
    Checksum,
    CliGeneral,
    CliMeta,
    Compress,
    Currency,
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

// TODO:
// extend this to have a json flag that will allow it to be piped into the next stage
pub fn find_cli_suggestons(
    desired_outcome: &DesiredOutcomes,
    mut writer: impl std::io::Write,
    os: &SupportedPlatforms,
) {
    let json_output = create_json_output(os);

    // default values for all platforms
    let wtto_info = json!("wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.");
    let currency = json!("btc, monero, wownero");
    let no_entry = json!(format!("No entry yet for that on {:?}, sorry. Feel free to open a PR for suggestions or email them to phoebx@pm.me or ironistdesign@pm.me", os));

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
        DesiredOutcomes::RandomGenerator => {
            json_output.get("random_generator").unwrap_or(&no_entry)
        }
        DesiredOutcomes::System => json_output.get("system").unwrap_or(&no_entry),
        DesiredOutcomes::VersionControl => json_output.get("version_control").unwrap_or(&no_entry),

        // hard coded entrees for all platforms
        DesiredOutcomes::Currency => &currency,
        DesiredOutcomes::WttuInfo => &wtto_info,
    };

    if let Err(e) = writeln!(writer, "{}", output) {
        eprint!("{:?}", e)
    }

}


fn create_json_output(os: &SupportedPlatforms) -> serde_json::Value {
    // TODO:
    // abstract this in such a way that it can be programatically added to.
    // I envision adding a suggestion is just running a function that updates the json structure
    // (but would this also allow updating the enum param possibilities?)

    let panic_msg = "create_json_output() - json data is malformed and pointing to the wrong platform. might need to open up the hood and fix the data's json file, or reach out to the dev";
    // println!("{:?}", os);
    // let supported_platforms = SupportedPlatforms::from_str(&capitalize(os)).unwrap();
    // println!("supported platform passed in: {:?}", supported_platforms);
    let json_output: serde_json::Value;
    match os {
        SupportedPlatforms::Linux => {
            let data = fs::read_to_string("./src/json_platform_data/linux-general.json")
                .expect("unable to read json file");
            let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
            let platform = &json["platform"];
            let linux = json!("linux");

            if platform == &linux {
                json_output = json!(&json);
                // something is messed up at the json level
            } else {
                panic!("{}", panic_msg)
            }
        }
        SupportedPlatforms::Mac => {
            let data = fs::read_to_string("./src/json_platform_data/mac.json")
                .expect("unable to read json file");
            let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
            let platform = &json["platform"];
            let mac = json!("mac");

            if platform == &mac {
                json_output = json!(&json);
            } else {
                panic!("{}", panic_msg)
            }
        }
        SupportedPlatforms::Windows => {
            let data = fs::read_to_string("./src/json_platform_data/windows.json")
                .expect("unable to read json file");
            let json: serde_json::Value = serde_json::from_str(&data).expect("json is malformed");
            let platform = &json["platform"];
            let windows = json!("windows");

            if platform == &windows {
                json_output = json!(&json);
            } else {
                panic!("{}", panic_msg)
            }
        }
        SupportedPlatforms::Unsupported => {
            json_output = json!("Unsupported platform, check out Linux for some options");
        }
    };

    json_output
}

pub fn underlining_os_to_enum(os: &str) -> SupportedPlatforms {
    match os {
        "linux" => SupportedPlatforms::Linux,
        "mac" => SupportedPlatforms::Mac,
        "windows" => SupportedPlatforms::Windows,
        _ => SupportedPlatforms::Unsupported,
    }
}



// TODO:
// find a better way to automate the testing instead of magic strings
#[test]
fn find_encode_suggeston() {
    // the writer
    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &SupportedPlatforms::Linux);
    assert_eq!(result, b"\"base64\"\n");

    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &SupportedPlatforms::Mac);
    assert_eq!(result, b"\"base64\"\n");

    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::Encode, &mut result, &SupportedPlatforms::Windows);
    assert_eq!(result, b"\"CertUtil\"\n");
}

#[test]
fn find_info_suggeston() {
    // the writer
    let wtto_byte_info :&[u8; 112]  = b"\"wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.\"\n";

    let mut result = Vec::new();
    find_cli_suggestons(&DesiredOutcomes::WttuInfo, &mut result, &SupportedPlatforms::Windows);
    assert_eq!(result, wtto_byte_info);
}
