pub mod express;
pub mod nextjs;

use std::collections::HashMap;

pub use express::express_method_endpoints;
pub use nextjs::nextjs_method_endpoints;

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let framework = detect_framework(&input_file);

    let methods = match framework.as_str() {
        "express" => express_method_endpoints(input_file),
        "nextjs" => nextjs_method_endpoints(input_file),
        _ => return Err("Framework not supported".to_string()),
    };

    Ok(methods)
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    if content.contains("express") {
        return "express".to_string();
    } else if content.contains("function GET") {
        return "nextjs".to_string();
    } else {
        return "unknown".to_string();
    }
}