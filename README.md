# Kitchen

A simple Rust CLI for simple Rust programs.

## Installation

It's easiest to install by cargo.

`cargo install kitchen-cli`

## What

Kitchen is a simple Rust CLI tool that aims to automate some of the tasks needed to run single file Rust applications built by `cargo new`. Right now, there are two commands.

- `kitchen-cli cleanup {foldername}` replaces your entire folder with `{foldername}.rs`, with the contents of the `src/main.rs`.
- `kitchen-cli create {filename}` turns the `{filename.rs}` back into the project!

## Why

As a novice in the Rust programming language, I find myself learning primarily through short programming problems. These solutions can almost always fit on a single file, so what choices are there to simply run a single rust file?

`rustc` and `cargo run` is a good option but it **fails** when it using dependencies such as the random module, giving an `unresolved import` error when you're not on `main.rs`.

`cargo new` is the default, but they make an entire folder to store a single file! It is overkill when you just want to store the code once you finish. Deleting and renaming files to solve this over and over again gets really annoying.

Maybe there's a better way I just didn't see, making this entire package obsolete. Nonetheless, a CLI tool is pretty fun to make.
