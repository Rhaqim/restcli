use std::{fs::File, io::Read};

pub fn read_file(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(file: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(file, content)
}
