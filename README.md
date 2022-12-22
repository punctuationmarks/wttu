# What Tool To Use
A simplistic tool for users who might be new or have forgotten which tools they should use for doing straight forward things around their system with offline first in mind. Currently it just prints general sugestions around what are some typical apps around Unix-esque systems and does not take account of the underlining OS. 

# Disclaimer
This is a work in progress. I don't develop on Windows or Mac often, so those tools might be out of date or missing, am looking into suggestions from other developers. Also, this will be a rolling release, so feel free to update often, contribute by opening a pull request or sending me an email with any suggestions

## Future Goals:
- keep with the mindset of having the tool's export be the input to something else, so return json option
- be somewhat opinionated, there are a million ways to do anything these days, what are the most common ways of solving the user's problem?
- have a simplistic UX


## Todo:
- Need to research the top tools for which problem to solve,
    would be nice if there was an api to just grab that from
- maybe look into this [great repo](https://github.com/agarrharr/awesome-cli-apps) by @agarrharr 

- Ideally be able to incorperate it into tldr or cheat, so the output of wttu can be used at the input,
    - Issues around this:
        - if there are 1 + x suggestions, which to render first?  
        - should the user have the option to pick which tool to push into tldr?
            - if so, is this index based?
    

## Current functionality
 
```
$ # install the tool from Crates.io
$ cargo install wttu

$ wttu <command> <platform>

$ wttu --help
$ wttu encrypt 
$ # passing optional OS, if not OS is passed, will grab OS being run on from std::env
$ wttu encrypt windows
$ wttu encrypt linux

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
