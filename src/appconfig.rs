use std::path::Path;
use std::process;

pub const DATABASE_FILE: &str = "./test.db";

pub fn check_dbfile(file_name: &str) {
    if !Path::new(&file_name).exists() {
        eprintln!("Can't find database {}", file_name);
        process::exit(1);
    }
}
