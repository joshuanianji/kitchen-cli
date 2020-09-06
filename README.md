# Kitchen

A simple Rust CLI for simple Rust programs. (A work in progress!)

## What

Kitchen is a simple Rust CLI tool that aims to automate some of the tasks needed to run single file Rust applications built by `cargo new`. Right now, I plan on making 2 commands:

- `kitchen cook` builds and runs your application. It essentially combines `cargo build` and `./target/debug/{folder_name}`
- `kitchen cleanup` replaces your entire folder with `main.rs` renamed to the folder name. Used when you finish the problem and just want to store the code. 


## Why

As a novice in the Rust programming language, I find myself learning primarily through programming problems. These solutions can almost always fit on a single file, so what choices are there to simply run a single rust file? 

`rustc` is a good option but it **fails** when it using packages such as the random module, giving an `unresolved import` error when you're not on `main.rs`.

`cargo new` is the default, but they make an entire folder to store a single file! It is overkill when you just want to store the code once you finish. Deleting and renaming files to solve this over and over again gets really annoying.

Maybe there's a better way I just didn't see, making this entire package obsolete. Nonetheless, a CLI tool is pretty fun to make.

