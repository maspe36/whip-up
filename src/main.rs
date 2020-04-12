extern crate proc_macro;

use clap::{Arg, App, ArgMatches};

use std::{env, process, fs};
use std::process::Command;
use std::io;
use std::io::Write;
use std::error::Error;
use syn::Block;
use syn::parse::Parser;
use std::path::PathBuf;

mod build;
mod utils;
mod whip;

static INPUT: &str = "input";

fn run(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let directory = PathBuf::from(args.value_of(INPUT).unwrap());

    println!("Searching for .whip file...");
    let whip_file = utils::find_whip_file(&directory)?;

    println!("Loading {}...", &whip_file.display().to_string());
    let whip_code = fs::read_to_string(&whip_file)?;

    println!("Bundling whip lib...");
    let whip_lib_str = include_str!("whip.rs");
    let whip_statements = Block::parse_within.parse_str(&whip_lib_str)?;

    println!("Performing lexical analysis on {}...", &whip_file.display().to_string());
    let statements = Block::parse_within.parse_str(&whip_code)?;

    println!("Compiling build executable...");
    let mut executable_path = build::compile(&whip_statements, &statements, &directory)?;

    if cfg!(windows) {
        // The binary ends with .exe on windows
        executable_path.set_extension("exe");
    }

    println!("Normalizing the path to the build executable...");
    executable_path = env::current_dir()?.join(executable_path);

    println!("Setting the current dir to {}...", &directory.display());
    env::set_current_dir(&directory)?;

    println!("Building whip target binary {}...", executable_path.display());
    let output = Command::new(executable_path)
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.success() {
        println!("\n\n**************\nSuccessfully built whip target")
    } else {
        println!("\n\n**************\nBuild for whip target failed")
    }

    Ok(())
}

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name(INPUT)
            .help("Sets the input directory to use")
            .required(true)
            .index(1))
        .get_matches();

    match run(&args) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e);
            println!("{:?}", env::current_dir());
            process::exit(1)
        }
    }
}
