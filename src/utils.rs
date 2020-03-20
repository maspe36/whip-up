use std::fs;

pub fn create_tmp_whip_dir() {
    let result = fs::create_dir("/tmp/whip-up/");

    match result {
        Ok(_) => println!("Created temporary whip dir"),
        Err(e) => println!("{}", e)
    }
}
