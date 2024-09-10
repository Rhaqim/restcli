pub mod gin;

use std::collections::HashMap;

pub use gin::gin_method_endpoints;

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, String>, String> {
    let framework = detect_framework(&input_file);

    let methods = match framework.as_str() {
        "gin" => gin_method_endpoints(input_file),
        _ => return Err("Framework not supported".to_string()),
    };

    Ok(methods)
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    if content.contains("gin.Default()") {
        return "gin".to_string();
    } else {
        return "unknown".to_string();
    }
}
