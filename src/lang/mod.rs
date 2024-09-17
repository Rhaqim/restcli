use std::collections::HashMap;

pub mod golang;
pub mod javascript;
pub mod python;
// pub mod rust;

use golang::process as go_process;
use python::process as py_process;
use javascript::process as js_process;

use crate::utils::file::get_file_extension;

// pub const COMMON_REGEX: &str = r"(?P<method>GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE)\s+(?P<path>\/[a-zA-Z0-9\/\-\_]+)";
// pub const COMMON_REGEX_2: &str = r"\b(?i)(POST|GET|DELETE|PATCH|PUT)\b.*?(/[\w/]*)";

pub fn process(input_files: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut input_content = HashMap::new();

    for input_file in input_files {
        let extension = get_file_extension(input_file).unwrap();

        let file_content = match extension {
            "go" => go_process(input_file).unwrap(),
            "py" => py_process(input_file).unwrap(),
            "js" => js_process(input_file).unwrap(),
            _ => {
                eprintln!("File extension not supported, supported extensions are: go, rs, py");
                std::process::exit(1);
            }
        };

        input_content.extend(file_content);
    }

    input_content
}
