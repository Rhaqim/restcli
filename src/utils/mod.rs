pub mod file;
pub mod regex;

pub use file::{is_supported_extension, read_file, write_file};
pub use regex::method_endpoints;
