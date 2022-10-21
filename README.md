# What Tool To Use
A simplistic tool for users who might be new or have forgotten which tools they should use for doing straight forward things around their system. 

## Goals:
- be platform respecting
    - if the binary is run on windows, mac, linux -  what tools to use where? it should automatically grab the underlining OS
- keep with the FreeBSD mindset of having the tool's export be the input to something else
- be opinionated, there are a million ways to do anything these days, what are the most common ways of solving the user's problem?
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



## Current functionality

```
$ cargo run compress

$ cargo run encrypt

```
