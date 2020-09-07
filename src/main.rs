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
use std::fs::OpenOptions;
use question::Question;

#[derive(StructOpt)]
#[structopt(about = "A simple Rust CLI for simple Rust apps!")]
struct Cli {
    #[structopt(help = "Either a `cleanup` or a `create`")]
    cmd: Cmd,
    #[structopt(help = "The path to the folder or file you want to run the command on.")]
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
                    Question::new("It looks like there are external dependencies in your program. Would you like to add dependency comments? These will help `kitchen create` if you choose to execute them later on. [Y/n]").default(Answer::YES).confirm();
                
                // Exits the program completely
                if let Answer::NO = answer {
                    eprintln!("Shutting down...");
                    exit(1);
                }

                // add comments to main file
                let main_file_name = &format!("{}/src/main.rs", args.path);
                let mut file = OpenOptions::new().append(true).open(main_file_name).expect("Couldn't open file!");

                // header
                write!(&mut file, "\n// KITCHEN DEPENDENCIES \n// These comments were generated automatically. Please do not tamper.").expect("Couldn't write comments to main.rs!");

                // add dependencies
                for dependency in dependencies {
                    write!(&mut file, "\n// {}", dependency).expect(&format!("Couldn't write {} to main.rs!", dependency));
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

            println!("Cleanup finished!");
        }
        Cmd::Create => {
            // if the input is "apples.rs", we'll get "apples"
            let proj_name: &str = args.path.split(".").next().unwrap();
            let proj_file: &str = &(proj_name.to_owned() + ".rs");

            println!("Creating cargo project from {}.rs...", proj_name);

            let _build = Command::new("cargo")
                .arg("new")
                .arg(proj_name)
                .output()
                .expect("Couldn't build project!");
            
            // copy contents of "{path}.rs" to "main.rs"
            let _mv = Command::new("mv")
                .arg(proj_file)
                .arg(&format!("{}/src/main.rs", proj_name))
                .output()
                .expect(&format!("Couldn't move contents of {}!", proj_file));
            
            // delete "{path}.rs"
            let _rm = Command::new("rm").arg(proj_file).output().expect(&format!("Couldn't delete {}!", proj_file));
        },
    }
}
