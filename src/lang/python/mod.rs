pub mod django;
pub mod flask;

use std::collections::HashMap;

pub use django::django_method_endpoints;
pub use flask::flask_method_endpoints;

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, String>, String> {
    let framework = detect_framework(&input_file);

    match framework.as_str() {
        "flask" => {
            let methods = flask_method_endpoints(input_file);
            Ok(methods)
        }
        "django" => {
            let methods = django_method_endpoints(input_file);
            Ok(methods)
        }
        _ => Err("Framework not supported".to_string()),
    }
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    if content.contains("Flask") {
        return "flask".to_string();
    }

    if content.contains("django.urls") {
        return "django".to_string();
    }

    "unknown".to_string()
}
