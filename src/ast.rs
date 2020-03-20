use std::{fs, io};

use syn;
use syn::{Block, Stmt};
use syn::parse::Parser;

pub fn parse(directory: String) -> syn::Result<Vec<Stmt>> {
    println!("Searching for .whip file in {} ...", directory);
    let whip_file = find_whip_file(directory).expect("No .whip file found");

    println!("Loading .whip file {} ...", whip_file);
    let code = fs::read_to_string(whip_file.clone()).expect("Unable to parse file");

    println!("Parsing .whip file {} ...", whip_file);
    Block::parse_within.parse_str(&code)
}

pub fn find_whip_file(directory: String) -> io::Result<String>{
    for entry in fs::read_dir(directory)? {
        let dir = entry?;
        if let Some(extension) = dir.path().extension() {
            if extension == "whip" {
                return Ok(dir.path().into_os_string().into_string().unwrap())
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "No .whip file found"))
}
