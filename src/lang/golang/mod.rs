pub mod gin;

use std::collections::HashMap;

pub use gin::gin_method_endpoints;

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, String>, String> {
    let framework = detect_framework(&input_file);

    match framework.as_str() {
        "gin" => {
            let methods = gin_method_endpoints(input_file);
            Ok(methods)
        }
        _ => Err("Framework not supported".to_string()),
    }
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    if content.contains("gin.Default()") {
        return "gin".to_string();
    }

    "unknown".to_string()
}
