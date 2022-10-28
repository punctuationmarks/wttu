#![deny(unused_crate_dependencies)]
use anyhow::Result;
use clap::{Parser, ValueEnum};
// use clap::Subcommand; // not entirely sure the usecase of a subcommand, but might be useful
use serde_json::json;
use std::env;

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
const WTTO_INFO :&str = "wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.";


fn main() -> Result<()> {
    let args = CliArgs::parse();
    match find_suggestons(&args.desired_outcome, &mut std::io::stdout(), OS) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }

    Ok(())
}

// TODO:
// extend this to have a json flag that will allow it to be piped into the next stage?
// need to think this through further
fn find_suggestons(
    desired_outcome: &DesiredOutcomes,
    mut writer: impl std::io::Write,
    os: &str
) -> Result<(), std::io::Error> {
    let json_output = create_json_output(os);


    let output = match desired_outcome {
        // TODO:
        // pick a lane and go with it
        // two different ways to acess the field values, just trying out the feel of it
        DesiredOutcomes::Checksum => &json_output.get("checksum").unwrap(),
        DesiredOutcomes::CliGeneral => &json_output.get("cli_general").unwrap(),
        DesiredOutcomes::CliMeta => &json_output.get("cli_meta").unwrap(),
        DesiredOutcomes::Compress => &json_output.get("compress").unwrap(),

        DesiredOutcomes::DocumentEditor => &json_output["document_editor"],
        DesiredOutcomes::Editor => &json_output["editor"],
        DesiredOutcomes::Encode => &json_output["encode"],
        DesiredOutcomes::Encrypt => &json_output["encrypt"],
        DesiredOutcomes::Find => &json_output["find"],
        DesiredOutcomes::Fingerprinting => &json_output["fingerprinting"],
        DesiredOutcomes::Firewall => &json_output["firewall"],
        DesiredOutcomes::Images => &json_output["images"],
        DesiredOutcomes::List => &json_output["list"],
        DesiredOutcomes::Manual => &json_output["manual"],
        DesiredOutcomes::Meta => &json_output["meta"],
        DesiredOutcomes::NavigateLocal => &json_output["navigate_local"],
        DesiredOutcomes::Networking => &json_output["networking"],
        DesiredOutcomes::PackageManager => &json_output["package_manager"],
        DesiredOutcomes::Partition => &json_output["partition"],
        DesiredOutcomes::Permission => &json_output["permissions"],
        DesiredOutcomes::Print => &json_output["print"],
        DesiredOutcomes::Privacy => &json_output["privacy"],
        DesiredOutcomes::RandomGenerator => &json_output["random_generator"],
        DesiredOutcomes::System => &json_output["system"],
        DesiredOutcomes::VersionControl => &json_output["version_control"],
        DesiredOutcomes::WttuInfo => &json_output["wttu_info"],
    };

    writeln!(writer, "{}", output)
}

fn create_json_output(os: &str) -> serde_json::Value {
    // TODO:
    // abstract this in such a way that it can be programatically added to.
    // I envision adding a suggestion is just running a function that updates the json structure
    // (but would this also allow updating the enum param possibilities?)

    let no_entry: String = format!("No entry yet for that on {}, sorry. Feel free to open a PR for suggestions or email them to phoebx@pm.me or ironistdesign@pm.me", OS);
    let json_output: serde_json::Value;
    if os == "windows" {
        json_output = json!({
            "checksum":&no_entry,
            "cli_general":&no_entry,
            "cli_meta":"powershell",
            "compress":&no_entry,
            "document_editor":"libreoff, notepad++, openoffice",
            "editor":"notepad++",
            "encode":&no_entry,
            "encrypt":&no_entry,
            "find":&no_entry,
            "fingerprinting":&no_entry, 
            "firewall":&no_entry,
            "images":&no_entry,
            "images_iso":&no_entry,
            "list":&no_entry,
            "manual":&no_entry,
            "meta": "tldr",
            "navigate_local":&no_entry,
            "networking":&no_entry,
            "package_manager":&no_entry,
            "passwords":&no_entry,
            "partition":&no_entry,
            "permissions":&no_entry,
            "print":&no_entry,
            "privacy": "tor",
            "random_generator":&no_entry,
            "system":&no_entry,
            "version_control":&no_entry,
            "not_sure_but_useful":&no_entry,
            "wttu_info": WTTO_INFO, 
        });
        // TODO:
        // find more mac specific applications, not just assuming all these are UNIX
    } else if os == "mac" {
        json_output = json!({
            "checksum":&no_entry,
            "cli_general": "clear, cp, scp",
            "cli_meta":"zsh",
            "compress": "gzip, tar, zip",
            "document_editor": "libreoffice, mdbook",
            "editor": "gedit, nano, neo-vim, vim",
            "encode": "base64",
            "encrypt": "fscrypt, gpg",
            "find": "find, grep, ripgrep",
            "fingerprinting": "latpot-detect", 
            "firewall": "ufw, firewall-cmd",
            "images": "imgp", // "pictures" like jpeg
            "images_iso": "docker, isoinfo", // "images" like iso, contianers
            "list": "dir, exa, ls, tree",
            "manual": "cheat, man, tldr",
            "meta": "cheat, man, tldr",
            "navigate_local": "cd, pwd",
            "networking": "arp, ifdown, ip, iw, ssh",
            "package_manager": "apt, flatpack, pacman",
            "passwords": "apg, hashcat",
            "partition": "fdisk, gparted, lvm, lvresize", // should logical volumes be here?
            "permissions": "chmod, chown, chroot",
            "print": "bat, cat, chafa, head",
            "privacy": "exif, tor",
            "random_generator": "apg",
            "system": "arch, free, fstrim, fuser, jobs, kexec, lsb_release, lsusb, lsof", 
            "version_control": "git",
            "wttu_info": WTTO_INFO, 

        });
    // Linux being the default
    } else {
        json_output = json!({
            "checksum":"md5sum, sha256sum",
            "cli_general": "clear, cp, scp",
            "cli_meta":"bash, zsh",
            "compress": "gzip, tar, zip",
            "document_editor": "cmp, codespell, libreoffice, mdbook",
            "editor": "cmp, codespell, gedit, nano, neo-vim, vim",
            "encode": "base64",
            "encrypt": "fscrypt, gpg, openssl",
            "find": "find, grep, ripgrep",
            "fingerprinting": "latpot-detect",
            "firewall": "ufw, firewall-cmd",
            // should this be plural? and should this extend to ISOs?
            "images": "imgp", // "pictures"
            "images_iso": "docker, isoinfo", // "images"
            "list": "dir, exa, ls, tree",
            "manual": "cheat, man, tldr",
            "meta": "cheat, man, tldr",
            "navigate_local": "cd, pwd",
            "networking": "arp, ifdown, ip, iw, ssh",
            "package_manager": "apt, flatpack, pacman",
            "passwords": "apg, hashcat",
            "partition": "fdisk, gparted, lvm, lvresize", // should logical volumes be here?
            "permissions": "chmod, chown, chroot",
            "print": "bat, cat, chafa, head",
            "privacy": "exif, tor",
            "random_generator": "apg",
            "system": "arch, free, fstrim, fuser, jobs, kexec, lsb_release, lsusb, lsof",
            "version_control": "cvs, git",

            // cmp <- compare two files byte vs byte
            // codespell <- find typos in a dir
            // exif <- metadata on jpef files
            // laptop-detect <- try to determine if on a laptop or desktop, whoa
            "not_sure_but_useful": "cmp, codespell, exif, laptop-detect",
            // todo: update this
            "wttu_info": WTTO_INFO, 

        });
    };
    return json_output;
}
#[test]
fn find_encode_suggeston() {
    // the writer
    let mut result = Vec::new();
    find_suggestons(&DesiredOutcomes::Encode, &mut result, &"linux");
    assert_eq!(result, b"\"base64\"\n");
}

#[test]
fn find_info_suggeston() {
    // the writer
    let WTTO_INFO_BYTE :&[u8; 112]  = b"\"wtto is a tool that aims to give decent suggestions for how to accomplish a known task. Run -h for more info.\"\n";

    let mut result = Vec::new();
    find_suggestons(&DesiredOutcomes::WttuInfo, &mut result, &"windows");
    assert_eq!(result, WTTO_INFO_BYTE);
}

