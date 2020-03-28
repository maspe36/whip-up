extern crate proc_macro;

use clap::{Arg, App};

use std::env;
use std::process::Command;
use std::io;
use std::io::Write;

mod build;
mod utils;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("input")
            .help("Sets the input directory to use")
            .required(true)
            .index(1))
        .get_matches();

    utils::create_tmp_whip_dir();

    let directory = String::from(matches.value_of("input").unwrap());
    let statements = build::parse(directory.clone()).unwrap();
    let intermediate_executable_path = build::compile(&statements, directory.clone());

    println!("Setting the current dir to the input dir...");
    let cd_result = env::set_current_dir(directory.clone());

    match cd_result {
        Err(io_err) => println!("{}", io_err),
        _ => {}
    }

    println!("Building whip target binary...");
    let output = Command::new(intermediate_executable_path)
        .env("PWD", directory.as_str())
        .output()
        .expect("Failed to build whip target");

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if output.status.success() {
        println!("\n\n**************\nSuccessfully built whip target")
    } else {
        println!("\n\n**************\nBuild for whip target failed")
    }
}
