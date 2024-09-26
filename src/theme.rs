use std::fs;
use std::io::{self};

pub fn list_themes(base_dir: &str) -> Result<(), io::Error> {
    let paths = fs::read_dir(base_dir)?;

    println!("Available themes:");

    for path in paths {
        let path = path?.path();

        if path.is_dir() {
            let num_files = fs::read_dir(&path)?.count();
            println!("- {} [{}]", path.file_name().unwrap().to_str().unwrap(), num_files);
        }
    };

    Ok(())
}
