# What Tool To Use
A simplistic tool for users who might be new or have forgotten which tools they should use for doing straight forward things around their system with offline first in mind. Currently it just prints general sugestions around what are some typical apps around Unix-esque systems and does not take account of the underlining OS. 

## Future Goals:
- be platform respecting
    - if the binary is run on windows, mac, linux -  what tools to use where? it should automatically grab the underlining OS
    - it would be nice to have a flag that could turn this feature off, since sometimes it's just nice to know some suggestions for offline systems.
- keep with the FreeBSD mindset of having the tool's export be the input to something else
- be somewhat opinionated, there are a million ways to do anything these days, what are the most common ways of solving the user's problem?
- have a simplistic UX 


## Todo:
- Need to research the top tools for which problem to solve,
    would be nice if there was an api to just grab that from

- Ideally be able to incorperate it into tldr, so the output of wttu can be used at the input of tldr,
    - Issues around this:
        - if there are 3 suggestions, which to render first? 
        - should the user have the option to pick which tool to push into tldr?
            - if so, is this index based?
        
- tree shake unused dependencies 

- need to go over the cli-apps-in-rust project to see how to execute the binary directly instead of running `cargo run command`


## Current functionality

```
$ # install the tool from Crates.io
$ cargo install wttu

$ wttu <command>

$ wttu --help
$ wttu version-control

```



## Contributing and publishing

- update the main.rs file, 
    - if extending current suggestions just update create_json_output() for desired OS
    - if adding new category
        - update DesiredOutcomes enum
        - update find_suggestions() json
        - update create_json_output

- update Cargo.toml file to next semver
- push to crates.io

```
$ cargo login <api token>
$ cargo publish 

```
