<!-- markdownlint-configure-file {
  "MD033": false,
  "MD041": false
} -->

# pshell

[![Crates.io](https://img.shields.io/crates/l/pshell)](https://github.com/a1ecbr0wn/pshell/blob/main/LICENSE) [![Crates.io](https://img.shields.io/crates/v/pshell)](https://crates.io/crates/pshell) [![Build Status](https://github.com/a1ecbr0wn/pshell/workflows/CI%20Build/badge.svg)](https://github.com/a1ecbr0wn/pshell/actions/workflows/build.yml) [![docs.rs](https://img.shields.io/docsrs/pshell)](https://docs.rs/pshell) [![dependency status](https://deps.rs/repo/github/a1ecbr0wn/pshell/status.svg)](https://deps.rs/repo/github/a1ecbr0wn/pshell)

`pshell` answers the question "Is my application running in a shell, and if so, which one?".

Example: you are installing something and want to make changes to the shell and you want to know what changes are required to which shell script.

<div align="center">

[Usage](#usage) - [Contribute](#contribute)

</div>

## Usage

Just a simple function that tells you whether the application is run from inside a shell:

``` rust
use pshell;

fn main() {
    // `find` returns the name of the shell in a string and the pid as a u32
    let (sh, pid) = pshell::find().unwrap_or(("unknown".to_string(), 0));
    println!("This application has been run from pid `{}`, which is a {} shell", pid, sh);
}
```

To try this out, and check it works OK on your OS/shell combination run the following from your shell:

``` bash
cargo run --example what_shell
```

## Why should you use this crate?

It is a small, simple crate that adds very little to your application size for discovering what shell this is running under by inspecting the name of the parent processes against a limited list of known shells.

## Why should you not use this crate?

You want an all-singing, all-dancing crate that identifies any knowns shell.

## Contribute

I have created a list of shells where this could be run from, it is not exhaustive, if your shell is not supported, feel free to raise an issue or a PR.
