use std::{fs::File, io::Read};

pub const SUPPORTED_EXTENSIONS: [&str; 3] = ["go", "rs", "py"];

pub fn read_file(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(file: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(file, content)
}

pub fn get_file_extension(file: &str) -> Option<&str> {
    std::path::Path::new(file)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
}

pub fn is_supported_extension(file: &Vec<String>) -> bool {
    for f in file {
        if !SUPPORTED_EXTENSIONS.contains(&get_file_extension(&f).unwrap_or("")) {
            return false;
        }
    }

    true
}

mod test {
    #[test]
    fn test_get_file_extension() {
        assert_eq!(super::get_file_extension("file.rs"), Some("rs"));
        assert_eq!(super::get_file_extension("file"), None);
    }

    #[test]
    fn test_is_supported_extension() {
        assert_eq!(
            super::is_supported_extension(&vec!["file.rs".to_string()]),
            true
        );
        assert_eq!(
            super::is_supported_extension(&vec!["file.go".to_string(), "file.rs".to_string()]),
            true
        );
        assert_eq!(
            super::is_supported_extension(&vec!["file".to_string()]),
            false
        );
    }
}
