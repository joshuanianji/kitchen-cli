use std::{
    fmt,
    fs::{self, OpenOptions, File},
    io::{prelude::*,BufReader},
    process::Command,
    str::FromStr,
};
use structopt::StructOpt;
extern crate question;
use question::Answer;
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
            println!("Cleaning up folder inside {}...", args.path);

            // status
            println!("Reading Cargo.toml file for possible dependeicies:");

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
                    if !(line.to_string().trim().len() == 0) {
                        // newline, ignore
                        println!("Dependency found! '{}'", line);
                        dependencies.push(line.to_string());
                    }
                } else {
                    if line == "[dependencies]" {
                        at_dependencis = true;
                    }
                }
            }

            // Warns user that there are external dependencies
            if dependencies.len() > 0 {
                let answer: Answer = 
                    Question::new("Dependencies found! Would you like to add dependency comments? These will help `kitchen create` if you choose to execute them later on. [Y/n]").default(Answer::YES).confirm();
                
                // Exits the program completely
                if let Answer::YES = answer {
                     // status
                    println!("Adding dependency comments:");

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
            }

            // copy the `main.rs` file to a `{folder}.rs` file
            println!("Copying {}/src/main.rs...", args.path);
            let _copy = Command::new("cp")
                .arg(format!("{}/src/main.rs", args.path))
                .arg(format!("{}.rs", args.path))
                .output()
                .expect("Couldn't main.rs file!");

            // delete everything in the folder
            println!("Deleting folder...");
            let _rm = Command::new("rm")
                .arg("-rf")
                .arg(args.path)
                .output()
                .expect("Couldn't delete folder!");

            println!("Cleanup finished!");
        }
        Cmd::Create => {
            // if the input is "apples.rs", proj_name will be "apples"
            // this is to make sure we don't fail if the user inputs the file name without the '.rs' ending.
            let proj_name: &str = args.path.split(".").next().unwrap();
            let proj_file: &str = &(proj_name.to_owned() + ".rs");

            println!("Creating cargo project from {}.rs...", proj_name);

            let _build = Command::new("cargo")
                .arg("new")
                .arg(proj_name)
                .output()
                .expect("Couldn't build project!");
            
            // reading kitchen dependencies
            println!("Reading Kitchen dependencies...");
            let mut dependencies: Vec<String> = Vec::new();
            let file = File::open(proj_file).expect("Couldn't open file!");
            let reader = BufReader::new(file);
            let mut at_dependencies = false;

            // this is memory intensive IF the file is too big. 
            for line in reader.lines() {
                // removing the Result (though it's probably a pretty bad way to do it)
                let str_line = line.expect("Couldn't read line!");
                if at_dependencies {
                    // push dependencies to vec.
                    dependencies.push(str_line.replace("//", "").trim().to_string());
                } else {
                    // Look for the comment
                    if str_line.trim() == "// These comments were generated automatically. Please do not tamper." {
                        at_dependencies = true;
                    }
                }
            }

            // delete "{path}.rs"
            println!("Deleting original file...");
            let _rm = Command::new("rm").arg(proj_file).output().expect(&format!("Couldn't delete {}!", proj_file));

            // THIS CODE IS HORRIBLE I HATE ITTTTT

            // remove kitchen dependencies from end of main.rs 
            println!("Removing Kitchen dependencies from main.rs...");
            let main_file_contents = fs::read_to_string(&format!("{}/src/main.rs", proj_name)).expect("Can't read main.rs!");
            let lines: Vec<_> = main_file_contents.lines().collect();

            // create will overrite the file
            let mut file = File::create(&format!("{}/src/main.rs", proj_name)).expect("Can't read main.rs!");
            for line in lines {
                if line.trim() == "// KITCHEN DEPENDENCIES" {
                    // break loop when we find this
                    break;
                }
                writeln!(file, "{}", line).expect("Can't write to Main.rs!");
            }

            // add dependencies to Cargo.toml
            println!("Adding dependencies to Cargo.toml...");
            let cargo_toml = &format!("{}/Cargo.toml", proj_name);
            let mut file = OpenOptions::new().append(true).open(cargo_toml).expect("Couldn't open Cargo.toml!");
            // write to Cargo.toml
            for dependency in dependencies {
                write!(&mut file, "\n{}", dependency).expect(&format!("Couldn't write {} to Cargo.toml!", dependency));
            }
            println!("Kitchen create complete!")
        }
    }
}
