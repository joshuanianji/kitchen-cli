use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;
use std::process::Command;
use std::str::FromStr;
use structopt::StructOpt;
extern crate question;
use question::Answer;
use question::Question;

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

            // Read Cargo.toml (for dependencies and stuff)
            let file = File::open(&format!("{}/Cargo.toml", args.path))
                .expect("Kitchen couldn't find a Cargo.toml file, and thus couldn't determine whether or not it's a Rust folder. Evacuating...");
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .expect("Couldn't load content of Cargo.toml!");

            let mut dependencies: Vec<String> = Vec::new();
            let mut at_dependencis = false;

            // Read through Cargo.toml and look at dependencies
            for line in contents.lines() {
                if at_dependencis {
                    println!("Dependency found! '{}'", line);
                    dependencies.push(line.to_string());
                } else {
                    if line == "[dependencies]" {
                        at_dependencis = true;
                    }
                }
            }

            // Warns user that there are external dependencies
            if dependencies.len() > 0 {
                let answer: Answer = 
                    Question::new("It looks like there are external dependencies in your program. Would you like to continue with the cleanup?").default(Answer::YES).confirm();
                
                // Exits the program completely
                if let Answer::NO = answer {
                    eprintln!("Shutting down...");
                    exit(1);
                }
            }

            // copy the `main.rs` file to a `{folder}.rs` file
            let _copy = Command::new("cp")
                .arg(format!("{}/src/main.rs", args.path))
                .arg(format!("{}.rs", args.path))
                .output()
                .expect("Couldn't main.rs file!");

            // delete everything in the folder
            let _rm = Command::new("rm")
                .arg("-rf")
                .arg(args.path)
                .output()
                .expect("Couldn't delete folder!");

            println!("Cleanup finished!")
        }
        Cmd::Create => println!("Create!"),
    }
}
