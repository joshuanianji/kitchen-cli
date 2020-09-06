use std::fmt;
use std::fs;
use std::process::Command;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    cmd: Cmd,
}

#[derive(Debug)]
enum Cmd {
    Cook,
    Cleanup,
}

#[derive(Debug)]
struct ParseCmdError;

impl fmt::Display for ParseCmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse Command Error")
    }
}

impl FromStr for Cmd {
    type Err = ParseCmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cook" => Ok(Cmd::Cook),
            "cleanup" => Ok(Cmd::Cleanup),
            _ => Err(ParseCmdError),
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let cargo_contents = fs::read_to_string("Cargo.toml")
        .expect("Could not read Cargo.toml! Is this a rust folder?");

    let mut name: String = String::new();

    for line in cargo_contents.lines() {
        if line.contains("name =") {
            name = line
                .trim()
                .split("=")
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .get(1)
                .expect("Couldn't read name of the folder!")
                .clone();
        }
    }

    println!("Kitchen is called {}", name);
    println!("Cmd is {:?}", args.cmd);
}
