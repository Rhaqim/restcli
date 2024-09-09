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

pub fn is_supported_extension(file: &str) -> bool {
    get_file_extension(file)
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

mod test {
    #[test]
    fn test_get_file_extension() {
        assert_eq!(super::get_file_extension("file.rs"), Some("rs"));
        assert_eq!(super::get_file_extension("file"), None);
    }

    #[test]
    fn test_is_supported_extension() {
        assert_eq!(super::is_supported_extension("file.rs"), true);
        assert_eq!(super::is_supported_extension("file.go"), true);
        assert_eq!(super::is_supported_extension("file.py"), true);
        assert_eq!(super::is_supported_extension("file"), false);
    }
}
