use clap::{Parser, ValueEnum};
use serde_json::json;
use std::fs;

/**
 * This mod creates Command Line suggestions. If extending to Gui, protocol, paradigm, language, etc etc, create addition mods and update accordingly.
 *
 * TODO:
 * Move all shared logic to another mod, so it will be easier to extend,
 */

/** ARGUMENTS */

// TODO:
// create help text and have autocomplete, where the user just types "enc" and then can hit tab to complete the enum option
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct WttuArgs {
    #[clap(value_enum, default_value_t=DesiredCliOutcomes::WttuInfo)]
    pub desired_outcome: DesiredCliOutcomes,

    // TODO!
    // need to think about the odering of these,
    // might be more practical to assume the user mainly wants to see what works
    // on that specific device and might want Gui vs cli interfaces more often
    // - might also be cool to have these be optional, but need to read more into clap
    // to see how to be able to specify the parameter vs just the set order here
    // bc currently the tool must be used:
    // wttu outcome platform interface
    // e.g. wttu compress windows Gui
    #[clap(value_enum)]
    pub platform: Option<SupportedPlatforms>,

    #[clap(value_enum)]
    pub interface: Option<Interface>,
}

// TODO:
// Remove any uncessary derives
#[derive(Clone, Debug, ValueEnum)]
pub enum Interface {
    Cli,
    Gui,
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
pub enum DesiredCliOutcomes {
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

/** Cli */

// TODO:
// extend this to have a json flag that will allow it to be piped into the next stage
pub fn find_cli_suggestons(
    desired_outcome: &DesiredCliOutcomes,
    mut writer: impl std::io::Write,
    os: &SupportedPlatforms,
) {
    let json_output = create_json_output(os);

    // default values for all platforms
    let wtto_info = json!("wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.");
    let currency = json!("btc, monero, wownero");
    let no_entry = json!(format!("No entry yet for that on {:?}, sorry. Feel free to open a PR for suggestions or email them to phoebx@pm.me or ironistdesign@pm.me", os));

    let output = match desired_outcome {
        DesiredCliOutcomes::Checksum => json_output.get("checksum").unwrap_or(&no_entry),
        DesiredCliOutcomes::CliGeneral => json_output.get("cli_general").unwrap_or(&no_entry),
        DesiredCliOutcomes::CliMeta => json_output.get("cli_meta").unwrap_or(&no_entry),
        DesiredCliOutcomes::Compress => json_output.get("compress").unwrap_or(&no_entry),
        DesiredCliOutcomes::DocumentEditor => {
            json_output.get("document_editor").unwrap_or(&no_entry)
        }
        DesiredCliOutcomes::Editor => json_output.get("editor").unwrap_or(&no_entry),
        DesiredCliOutcomes::Encode => json_output.get("encode").unwrap_or(&no_entry),
        DesiredCliOutcomes::Encrypt => json_output.get("encrypt").unwrap_or(&no_entry),
        DesiredCliOutcomes::Find => json_output.get("find").unwrap_or(&no_entry),
        DesiredCliOutcomes::Fingerprinting => {
            json_output.get("fingerprinting").unwrap_or(&no_entry)
        }
        DesiredCliOutcomes::Firewall => json_output.get("firewall").unwrap_or(&no_entry),
        DesiredCliOutcomes::Images => json_output.get("images").unwrap_or(&no_entry),
        DesiredCliOutcomes::List => json_output.get("list").unwrap_or(&no_entry),
        DesiredCliOutcomes::Manual => json_output.get("manual").unwrap_or(&no_entry),
        DesiredCliOutcomes::Meta => json_output.get("meta").unwrap_or(&no_entry),
        DesiredCliOutcomes::NavigateLocal => json_output.get("navigate_local").unwrap_or(&no_entry),
        DesiredCliOutcomes::Networking => json_output.get("networking").unwrap_or(&no_entry),
        DesiredCliOutcomes::PackageManager => {
            json_output.get("package_manager").unwrap_or(&no_entry)
        }
        DesiredCliOutcomes::Partition => json_output.get("partition").unwrap_or(&no_entry),
        DesiredCliOutcomes::Permission => json_output.get("permissions").unwrap_or(&no_entry),
        DesiredCliOutcomes::Print => json_output.get("print").unwrap_or(&no_entry),
        DesiredCliOutcomes::Privacy => json_output.get("privacy").unwrap_or(&no_entry),
        DesiredCliOutcomes::RandomGenerator => {
            json_output.get("random_generator").unwrap_or(&no_entry)
        }
        DesiredCliOutcomes::System => json_output.get("system").unwrap_or(&no_entry),
        DesiredCliOutcomes::VersionControl => {
            json_output.get("version_control").unwrap_or(&no_entry)
        }

        // hard coded entrees for all platforms
        DesiredCliOutcomes::Currency => &currency,
        DesiredCliOutcomes::WttuInfo => &wtto_info,
    };

    if let Err(e) = writeln!(writer, "{}", output) {
        eprint!("{:?}", e)
    }
}

/** Cli Helper functions */

fn create_json_output(os: &SupportedPlatforms) -> serde_json::Value {
    let json_output: serde_json::Value = match os {
        SupportedPlatforms::Linux => {
            let data = fs::read_to_string("./src/json_platform_data/linux-general.json")
                .expect("unable to read json file");
            let json_data: serde_json::Value =
                serde_json::from_str(&data).expect("json is malformed");

            return_json_output(&json_data["platform"], json!("linux"), &json_data)
        }
        SupportedPlatforms::Mac => {
            let data = fs::read_to_string("./src/json_platform_data/mac.json")
                .expect("unable to read json file");

            let json_data: serde_json::Value =
                serde_json::from_str(&data).expect("json is malformed");
            return_json_output(&json_data["platform"], json!("mac"), &json_data)
        }
        SupportedPlatforms::Windows => {
            let data = fs::read_to_string("./src/json_platform_data/windows.json")
                .expect("unable to read json file");

            let json_data: serde_json::Value =
                serde_json::from_str(&data).expect("json is malformed");
            return_json_output(&json_data["platform"], json!("windows"), &json_data)
        }
        // for cases when they run a command on the underlining OS and nothing shows up
        SupportedPlatforms::Unsupported => {
            json!("Unsupported platform, check out Linux for some options")
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

fn return_json_output(
    platform: &serde_json::Value,
    json_platform: serde_json::Value,
    json: &serde_json::Value,
) -> serde_json::Value {
    let panic_msg = "create_json_output() - json data is malformed and pointing to the wrong platform. might need to open up the hood and fix the data's json file, or reach out to the dev";
    if platform == &json_platform {
        json!(&json)
    } else {
        panic!("{}", panic_msg)
    }
}

// just a simple helper
fn _print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/** Cli Tests */

// TODO:
// find a better way to automate the testing instead of magic strings
#[test]
fn find_encode_suggeston() {
    // the writer
    let mut result = Vec::new();
    find_cli_suggestons(
        &DesiredCliOutcomes::Encode,
        &mut result,
        &SupportedPlatforms::Linux,
    );
    assert_eq!(result, b"\"base64\"\n");

    let mut result = Vec::new();
    find_cli_suggestons(
        &DesiredCliOutcomes::Encode,
        &mut result,
        &SupportedPlatforms::Mac,
    );
    assert_eq!(result, b"\"base64\"\n");

    let mut result = Vec::new();
    find_cli_suggestons(
        &DesiredCliOutcomes::Encode,
        &mut result,
        &SupportedPlatforms::Windows,
    );
    assert_eq!(result, b"\"CertUtil\"\n");
}

#[test]
fn find_info_suggeston() {
    // the writer
    let wtto_byte_info :&[u8; 112]  = b"\"wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.\"\n";

    let mut result = Vec::new();
    find_cli_suggestons(
        &DesiredCliOutcomes::WttuInfo,
        &mut result,
        &SupportedPlatforms::Windows,
    );
    assert_eq!(result, wtto_byte_info);
}
