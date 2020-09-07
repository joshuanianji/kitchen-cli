use std::fmt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    cmd: Cmd,
    path: String,
}

#[derive(Debug)]
enum Cmd {
    Cleanup,
    Create,
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
            "cleanup" => Ok(Cmd::Cleanup),
            "create" => Ok(Cmd::Create),
            _ => Err(ParseCmdError),
        }
    }
}

fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Cmd::Cleanup => {
            println!(
                "Path to Cargo.toml is suggested to be: {}/Cargo.toml",
                args.path
            );

            // Check if it's a Rust folder (containing Cargo.toml)
            if Path::new(&format!("{}/Cargo.toml", args.path)).exists() {
                // copy the `main.rs` file to a `{folder}.rs` file
                let copy = Command::new("cp")
                    .arg(format!("{}/src/main.rs", args.path))
                    .arg(format!("{}.rs", args.path))
                    .output()
                    .expect("Couldn't main.rs file!");

                // delete everything in the folder
                let rm = Command::new("rm")
                    .arg("-rf")
                    .arg(args.path)
                    .output()
                    .expect("Couldn't delete folder!");

                println!("pog?")
            } else {
                println!("Kitchen couldn't find a Cargo.toml file, and thus couldn't determine whether or not it's a Rust folder. Evacuating...")
            }
        }
        Cmd::Create => println!("Create!"),
    }
}
