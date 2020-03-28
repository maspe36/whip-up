use std::{fs, io};

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

pub fn create_tmp_whip_dir() {
    let result = fs::create_dir("/tmp/whip-up/");

    match result {
        Ok(_) => println!("Created temporary whip dir"),
        Err(e) => println!("Error creating whip dir: {}", e)
    }
}
