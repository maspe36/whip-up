use std::{fs, io};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;

pub static TMP_DIR: &str = ".whip-up/";

/// Finds the first .whip file found within a directory
pub fn find_whip_file(directory: &PathBuf) -> io::Result<PathBuf> {
    use std::io::{Error, ErrorKind};

    fs::read_dir(directory)?
        .filter_map(Result::ok)
        .filter(|e| e.path().extension() == Some(OsStr::new("whip")))
        .map(|e| e.path())
        .next()
        .ok_or(Error::new(ErrorKind::NotFound,format!("No .whip file found in {}", directory.display())))
}

/// Writes out a temp file of the source code for the build executable
pub fn write_tmp_file(code: &str, directory: &PathBuf) -> io::Result<PathBuf> {
    let tmp_dir_path = directory.join(TMP_DIR);

    fs::create_dir(&tmp_dir_path)?;

    let file_path = tmp_dir_path.join("main.rs");

    println!("Writing intermediate file {} ...", file_path.display());
    let mut file = File::create(&file_path)?;
    file.write_all(code.as_bytes())?;

    Ok(file_path)
}
