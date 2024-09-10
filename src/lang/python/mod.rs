pub mod fastapi;
pub mod flask;

use std::collections::HashMap;

pub use fastapi::fastapi_method_endpoints;
pub use flask::flask_method_endpoints;

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, String>, String> {
    let framework = detect_framework(input_file);

    let methods = match framework.as_str() {
        "flask" => flask_method_endpoints(input_file),
        "fastapi" => fastapi_method_endpoints(input_file),
        _ => return Err("Framework not supported".to_string()),
    };

    Ok(methods)
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).unwrap_or_else(|_| "".to_string());

    if content.contains("Flask") {
        "flask".to_string()
    } else if content.contains("FastAPI") {
        "fastapi".to_string()
    } else {
        "unknown".to_string()
    }
}
