pub mod django;
pub mod flask;

use std::collections::HashMap;

use regex::Regex;

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

/// Python file
/// Flask framework
/// method is in the form of @app.route
/// Flask framework Regex::new(r#"\b@app\.route\(\"([^\"]+)\"\)"#).unwrap();
///
/// Django framework
/// method is in the form of path, re_path, include
/// Django framework Regex::new(r#"\b(path|re_path|include)\(\"([^\"]+)\"\)"#).unwrap();
///
/// FastAPI framework
/// method is in the form of @app.get, @app.post, @app.delete, @app.put
/// FastAPI framework Regex::new(r#"\b@app\.get\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.post\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.delete\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.put\(\"([^\"]+)\"\)"#).unwrap();
///
pub fn detect_methods_in_py_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b@app\.route\(\"([^\"]+)\"\)"#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let endpoint = caps.get(1).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, "GET", endpoint);

        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}
