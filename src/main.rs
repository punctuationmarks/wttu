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
    Cli,
    Compress,
    DocumentEditor,
    Editor,
    Encode,
    Encrypt,
    Find,
    Firewall,
    Images,
    List,
    Manual,
    // TODO: rename
    NavigateLocal,
    Networking,
    PackageManager,
    Partition,
    Permission,
    Print, // print to the console
    RandomGenerator,
    System,
    VersionControl
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
fn find_suggestons(
    desired_outcome: &DesiredOutcomes,
    mut writer: impl std::io::Write,
) -> Result<(), std::io::Error> {
    let json_output = create_json_output();
    // TODO:
    // this returns a result that doesn't handle all errors, need to read the book a bit more
    // to find out how to add error handling to match statements
    let output = match desired_outcome {
        // two different ways to acess the field values
        DesiredOutcomes::Cli => &json_output.get("cli").unwrap(),
        DesiredOutcomes::Compress => &json_output.get("compress").unwrap(),
        // should this have subcommands? document_editor ide_editor?
        DesiredOutcomes::Editor => &json_output["editor"],
        DesiredOutcomes::DocumentEditor => &json_output["document_editor"],

        DesiredOutcomes::Encode => &json_output["encode"],
        DesiredOutcomes::Encrypt => &json_output["encrypt"],
        DesiredOutcomes::Find => &json_output["find"],
        DesiredOutcomes::Firewall => &json_output["firewall"],
        DesiredOutcomes::Images => &json_output["images"],
        // prints the entire nested json
        // DesiredOutcomes::List => &json_output["list"],
        // prints just the applications
        DesiredOutcomes::List => &json_output["list"]["apps"],
        DesiredOutcomes::Manual => &json_output["manual"],
        DesiredOutcomes::NavigateLocal => &json_output["navigate_local"],
        DesiredOutcomes::Networking => &json_output["networking"],
        DesiredOutcomes::PackageManager => &json_output["package_manager"],
        DesiredOutcomes::Partition => &json_output["partition"],
        DesiredOutcomes::Permission => &json_output["permissions"],
        DesiredOutcomes::Print => &json_output["print"],
        DesiredOutcomes::RandomGenerator => &json_output["random_generator"],
        DesiredOutcomes::System => &json_output["system"],
        DesiredOutcomes::VersionControl => &json_output["version_control"],
        // no need for having _ since it's an enum
    };

    writeln!(writer, "{}", output)
}

fn create_json_output() -> serde_json::Value {
    // TODO:
    // abstract this in such a way that it can be programatically added to.
    // I envision adding a suggestion is just running a function that updates the json structure
    // (but would this also allow updating the enum param possibilities?)
    let json_output: serde_json::Value = json!({
        "cli": "clear, cp, scp, ",
        "compress": "gzip, tar, zip",
        "document_editor": "libreoffice, mdbook",
        "editor": "gedit, nano, neo-vim, vim",
        "encode": "base64",
        "encrypt": "fscrypt, gpg",
        "find": "find, grep, ripgrep",
        "firewall": "ufw, firewall-cmd",
        // should this be plural? and should this extend to ISOs?
        "images": "imgp", // "pictures"
        "images_iso": "docker, isoinfo", // "images"
        // not sure if this is the correct way to go about this, could be a helper function in clap for --help on each
        "list": {
            "description":"list everything in a directory",
            "apps":"dir, exa, ls, tree"
        },
        "manual": "cheat, man, tldr",
        "navigate_local": "cd, pwd",
        "networking": "arp, ifdown, ip, iw, ssh",
        "package_manager": "apt, flatpack, pacman",
        "passwords": "apg, hashcat",
        "partition": "fdisk, gparted, lvm, lvresize", // should logical volumes be here?
        "permissions": "chmod, chown, chroot",
        "print": "bat, cat, chafa, head",
        "random_generator": "apg",
        "system": "arch, free, fstrim, fuser, jobs, kexec, lsb_release, lsusb, lsof",
        // look up other vc standards,
        "version_control": "git",

        // cmp <- compare two files byte vs byte
        // codespell <- find typos in a dir
        // exif <- metadata on jpef files
        // laptop-detect <- try to determine if on a laptop or desktop, whoa
        "not_sure_but_useful": "cmp, codespell, exif, laptop-detect",

    });
    return json_output;
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
